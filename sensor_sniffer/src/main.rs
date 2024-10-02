use reqwest::blocking::Client;
use std::collections::VecDeque;
use std::sync::OnceLock;
use std::io::{BufRead, BufReader, Read, Result};
use std::fs::File;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use regex::Regex;
use apache_avro::{Schema, Writer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
extern crate hex;
#[macro_use]
extern crate ini;

const CONFIG_PATH: &str = "./config/config.ini";
const AVRO_SCHEMA_PATH: &str = "./avro/schema.avsc";
const OUI_LOOKUP_PATH: &str = "./config/oui.txt";
static SERIAL_ID: OnceLock<u32> = OnceLock::new();
static PACKET_BUFFER_SIZE: OnceLock<i32> = OnceLock::new();
static API_ENDPOINT: OnceLock<String> = OnceLock::new();
static HEARTBEAT_FREQ: OnceLock<u32> = OnceLock::new();
static AVRO_SCHEMA: OnceLock<Schema> = OnceLock::new();
static OUI_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

#[derive(Debug, Clone, serde::Serialize)]
pub struct BLEPacket {
    pub timestamp: f64,                // Packet timestamp in seconds
    pub rssi: i32,                     // Received signal strength indication
    pub channel_index: i32,            // BLE channel index (0-39)
    pub advertising_address: i64,      // BLE device adv address
    pub company_id: i32,               // Company identifier from advertisement
    pub packet_counter: i64,           // Packet counter from sensor
    pub protocol_version: i32,         // Version of protocol
    pub power_level: i32,              // Power level of the packet
    pub oui: String,                   // Manufacturer based on MAC address
    pub long_device_name: String,      // Device's chosen name from adv data
    pub short_device_name: String     // Device's shortened name
}

fn parse_oui_file(file_path: &str) -> Result<HashMap<String, String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut oui_map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("(hex)") {
            // Example line: "00-00-0A   (hex)    CISCO SYSTEMS, INC."
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() > 2 {
                let oui = parts[0].replace("-", ":");
                let company_name = parts[2..].join(" ");
                oui_map.insert(oui, company_name);
            }
        }
    }

    Ok(oui_map)
}

// loads program constants from INI file
fn load_config() {
    // Load the INI file by creating a map of the contents and setting our globals
    let map: HashMap<String, HashMap<String, Option<String>>> = ini!(CONFIG_PATH);
    // println!("{:#?}", map);

    SERIAL_ID
        .set(map["settings"]["serial_id"].clone().unwrap().parse::<u32>().unwrap())
        .unwrap();

    PACKET_BUFFER_SIZE
        .set(map["settings"]["packet_buffer_size"].clone().unwrap().parse::<i32>().unwrap())
        .unwrap();

    API_ENDPOINT
        .set(map["settings"]["api_endpoint"].clone().unwrap())
        .unwrap();

    HEARTBEAT_FREQ
        .set(map["settings"]["heartbeat_freq"].clone().unwrap().parse::<u32>().unwrap())
        .unwrap();

    // load the avro schema into a schema obj for serialization
    let mut schema_file: File = File::open(AVRO_SCHEMA_PATH).expect("Unable to open avro schema file");
    let mut schema_str: String = String::new();
    schema_file.read_to_string(&mut schema_str).expect("Unable to read schema file");
    let schema: Schema = Schema::parse_str(&schema_str).expect("Unable to parse avro schema");

    AVRO_SCHEMA
        .set(schema)
        .unwrap();

    // load the OUI map so we can provide that information
    if OUI_MAP.set(parse_oui_file(OUI_LOOKUP_PATH).unwrap()).is_err() {
        eprintln!("Failed to initialize OUI map");
    }
}

// Starts the NRFutil ble-sniffer tool for capturing BLE packets
fn start_nrf_sniffer(interface: &String) -> Child {
    let sniffer: Child = Command::new("nrfutil")
        .args(&[
            "ble-sniffer",
            "sniff", // call sniffer
            "--port",
            interface,             // sniff on interface we detected it on
            "--log-output=stdout", // send logs to stdout
            "--json",              // so output is formatted
            "--log-level",
            "debug", // so we can see packets in stdout
            "--output-pcap-file",
            "NUL", // trick nrf into running but not creating its pcapng
        ])
        .stdout(Stdio::piped()) // pipe stdout so rust can capture and process it
        .spawn() // spawn the process
        .expect("Failed to start nrfutil.");
    println!("nrfSniffer started with PID: {}", sniffer.id());
    return sniffer; // return process so we can reference it later
}

// take in a string of hex values that is the advertising payload of a BLE packet and parse it to get attributes from it
fn parse_advertising_data(advertising_data_hex: &str) -> HashMap<String, String> {
    let advertising_data: Vec<u8> = hex::decode(advertising_data_hex)
        .expect("Failed to decode advertising data hex string");

    let mut index: usize = 0;
    let mut result: HashMap<String, String> = HashMap::new();

    // Insert default values in case info is not provided in the adv data (common)
    result.insert("long_device_name".to_string(), "Unknown".to_string());
    result.insert("short_device_name".to_string(), "Unknown".to_string());
    result.insert("company_id".to_string(), "-1".to_string());
    result.insert("power_level".to_string(), "-1".to_string());

    // check the values for 'indicators'
    while index < advertising_data.len() {
        let length: usize = advertising_data[index] as usize;
        if length == 0 {
            break;
        }

        let ad_type: u8 = advertising_data[index + 1];
        let ad_data: &[u8] = &advertising_data[index + 2..index + length + 1];

        match ad_type {
            0x09 => {
                // Complete Local Name
                if let Ok(name) = String::from_utf8(ad_data.to_vec()) {
                    result.insert("long_device_name".to_string(), name);
                }
            }
            0x08 => {
                // Shortened Local Name
                if let Ok(name) = String::from_utf8(ad_data.to_vec()) {
                    result.insert("short_device_name".to_string(), name);
                }
            }
            0xFF => {
                // Manufacturer Specific Data
                if ad_data.len() >= 2 {
                    let company_id: i32 = (ad_data[1] as i32) << 8 | (ad_data[0] as i32);
                    result.insert("company_id".to_string(), company_id.to_string());
                }
            }
            0x0A => {
                // TX Power Level
                result.insert("power_level".to_string(), (ad_data[0] as i8).to_string());
            }
            _ => {
                // Other data types can be handled here
            }
        }

        index += length + 1;
    }

    result
}

// take in a mac and return the OUI
fn lookup_oui(mac_address: i64) -> String {
    let oui_prefix: String = format!(
        "{:02X}:{:02X}:{:02X}",
        (mac_address >> 40) & 0xFF,
        (mac_address >> 32) & 0xFF,
        (mac_address >> 24) & 0xFF
    );

    // Use the map and return either the company name or "Unknown"
    OUI_MAP.get()
        .and_then(|map: &HashMap<String, String> | map.get(&oui_prefix))
        .unwrap_or(&"Unknown".to_string())
        .clone() 
}

// parse the log statement from nrfutil
fn parse_ble_packet(input: &str) -> BLEPacket {
    // use regex to extract the data from the log statement
    let timestamp_re: Regex = Regex::new(r"fw_timestamp:\s(\d+)").unwrap();
    let rssi_re: Regex = Regex::new(r"rssi_sample:\s([-]?\d+)").unwrap();
    let channel_index_re: Regex = Regex::new(r"channel_index:\s(\d+)").unwrap();
    // mac addresses are missing leading 0s for some reason...
    let advertising_address_re: Regex = Regex::new(r"advertising_address:\sBleAddress\(((?:[0-9A-Fa-f]{1,2}[:-]){5}[0-9A-Fa-f]{1,2})(?:\s[\w]*)?\)").unwrap();
    let packet_counter_re: Regex = Regex::new(r"packet_counter:\s(\d+)").unwrap();
    let protocol_version_re: Regex = Regex::new(r"protocol_version:\sVersionX\((\d+)\)").unwrap();
    let adv_data_re: Regex = Regex::new(r"data: AdvData\(\[([\d, ]+)\]\)").unwrap();

    let timestamp: f64 = timestamp_re
        .captures(input)
        .and_then(|cap: regex::Captures<'_>| cap.get(1).map(|m: regex::Match<'_>| m.as_str().parse::<f64>().ok()))
        .flatten()
        .unwrap_or(0.0); // Default to 0.0 if parsing fails

    let rssi: i32 = rssi_re
        .captures(input)
        .and_then(|cap: regex::Captures<'_>| cap.get(1).map(|m: regex::Match<'_>| m.as_str().parse::<i32>().ok()))
        .flatten()
        .unwrap_or(-1); // Default to -1 if parsing fails

    let channel_index: i32 = channel_index_re
        .captures(input)
        .and_then(|cap: regex::Captures<'_>| cap.get(1).map(|m: regex::Match<'_>| m.as_str().parse::<i32>().ok()))
        .flatten()
        .unwrap_or(-1); // Default to -1 if parsing fails

    let advertising_address: i64 = if let Some(caps) = advertising_address_re.captures(input) {
        let mac_str: &str = &caps[1]; // Capture the MAC address string
        
        // Split the MAC address into parts and parse as a vector of u8
        let mac_address: Vec<u8> = mac_str.split(|c: char| c == ':' || c == '-')
            .filter_map(|part: &str| u8::from_str_radix(part, 16).ok())
            .collect();

        // Convert the MAC address bytes to a single i64
        mac_address.iter().fold(0, |acc, &byte| (acc << 8) | byte as i64)
    } else {
        -1 // Default to -1 if parsing fails
    };

    let packet_counter: i64 = packet_counter_re
        .captures(input)
        .and_then(|cap: regex::Captures<'_>| cap.get(1).map(|m: regex::Match<'_>| m.as_str().parse::<i64>().ok()))
        .flatten()
        .unwrap_or(-1); // Default to -1 if parsing fails

    let protocol_version: i32 = protocol_version_re
        .captures(input)
        .and_then(|cap: regex::Captures<'_>| cap.get(1).map(|m: regex::Match<'_>| m.as_str().parse::<i32>().ok()))
        .flatten()
        .unwrap_or(-1); // Default to -1 if parsing fails

    let adv_data: &str = adv_data_re
        .captures(input)
        .and_then(|cap: regex::Captures<'_>| cap.get(1).map(|m: regex::Match<'_>| m.as_str()))
        .unwrap_or(""); // Default to empty if parsing fails

    // initialize to defaults in case no advertising data presented
    let mut power_level: i32 = -1;
    let mut company_id: i32 = -1;
    let mut long_device_name: String = "Unknown".to_string();
    let mut short_device_name: String = "Unknown".to_string();
    if adv_data != "" {
        let hex_adv_data: Vec<u8> = adv_data
            .split(',')
            .map(|s: &str| s.trim().parse::<u8>().expect("Invalid input"))
            .collect();
        let adv_hex_string: String = hex_adv_data
            .iter()
            .map(|b: &u8| format!("{:02x}", b))
            .collect::<String>();

        let parsed_adv_data: HashMap<String, String> = parse_advertising_data(&adv_hex_string);
        power_level = parsed_adv_data["power_level"].parse::<i32>().ok().unwrap_or(-1);
        long_device_name = parsed_adv_data["long_device_name"].to_string();
        short_device_name = parsed_adv_data["short_device_name"].to_string();
        company_id = parsed_adv_data["company_id"].parse::<i32>().ok().unwrap_or(-1);
    } 

    let oui: String = lookup_oui(advertising_address);

    BLEPacket {
        timestamp,
        rssi,
        channel_index,
        advertising_address,
        company_id,
        packet_counter,
        protocol_version,
        power_level,
        oui,
        long_device_name,
        short_device_name
    }

}

// use the schema to encode/serialize a BLEPacket to avro
fn encode_avro(packet: BLEPacket) -> Vec<u8> {
    let mut writer: Writer<'_, Vec<u8>> = Writer::new(&AVRO_SCHEMA.get().unwrap(), Vec::new());
    writer.append_ser(packet).expect("Unable to serialize data");
    let encoded_data: Vec<u8> = writer.into_inner().expect("Unable to get encoded data");

    encoded_data
}

// release encoded packets to the api
fn offload_to_api(queue: &mut VecDeque<Vec<u8>>) {
    println!("Offloading to API!");

    // create object to offload via API - its the first PACKET_BUFFER_SIZE packets of the queue
    let mut data_to_send: Vec<Vec<u8>> = Vec::new();
    for _ in 0..*PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
        if let Some(item) = queue.pop_front() {
            data_to_send.push(item);
        }
    }

    // maybe move client creation to global so it isn't made every time this is called
    let __client: Client = Client::new();
    // send the dequeued packets to API
    // let response = client.post(api_url)
    //     .body(buffer)
    //     .header("Content-Type", "application/octet-stream")
    //     .send();

    // match response {
    //     Ok(resp) => println!("File sent successfully: {}, Response: {:?}", file_name, resp),
    //     Err(err) => eprintln!("Failed to send file: {}, Error: {}", file_name, err),
    // }
}

// constantly take in the output of nrfutil. stop with an interrupt. manage api sending.
fn parse_offload(running: Arc<AtomicBool>, interface: &String) {
    let mut packet_queue: VecDeque<Vec<u8>> = VecDeque::new(); // queue to hold data
    let mut sniffer: Child = start_nrf_sniffer(interface); // start sniffer

    // we do this loop forever (or until interrupt)
    while running.load(Ordering::SeqCst) {
        // capture the nrf info from stdout
        if let Some(stdout) = sniffer.stdout.take() {
            let reader = BufReader::new(stdout);
            // Read the stdout line by line as it comes in
            for line in reader.lines() {
                let line: String = line.expect("Could not read line from stdout");
                // println!("{}", line.clone());
                if line.contains("Parsed packet") {
                    let parsed_ble_packet: BLEPacket = parse_ble_packet(&line); 
                    let encoded_packet: Vec<u8> = encode_avro(parsed_ble_packet.clone());
                    println!("\n\n{}", line);
                    println!("{:#?}", parsed_ble_packet);
                    // println!("{:?}", encoded_packet);
                    packet_queue.push_back(encoded_packet);
                    // println!("Queue Size: {}", queue.len());
                }

                if packet_queue.len() >= *PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
                    offload_to_api(&mut packet_queue); // by reference so offload can empty queue FIFO
                }
            }
        }
    }
    // dump_queue(); if shutting down, might want to dump the queue to api first to avoid data loss?
}

// detect the COM port/interface the sniffer connected to
fn get_interface() -> String {
    let output = Command::new("nrfutil")
        .args(&["device", "list"])
        .output()
        .expect("Failed to run nrfutil device list");

    let output_str: = String::from_utf8_lossy(&output.stdout);

    // Find the line that starts with "ports"
    for line in output_str.lines() {
        if line.starts_with("ports") {
            // Extract port info (ex. COM3) from the line
            let parts: Vec<&str> = line.split_whitespace().collect();
            return parts[1].to_string(); // port num is the second part (ex. ports    COM3)
        }
    }
    panic!("No valid interface found.");
}

fn main() {
    println!("\nLoading Config...\n");
    load_config();

    println!("\nStarting Sensor (Serial: {})!\n", SERIAL_ID.get().unwrap());
    // auto detects what port the sniffer identified itself as
    let interface: String = get_interface();
    println!("\nNRF Dongle detected on port: {}\n", interface);

    // atomic boolean to track if the program should stop
    let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    // ctrl c handler - so the program will exit the infinite loop
    {
        let r: Arc<AtomicBool> = running.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl+C Interrupt Received, shutting down...");
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    // capture packets, parse them, and periodically send to api
    parse_offload(running.clone(), &interface);

    println!("\nSensor shut down.\n");
}
