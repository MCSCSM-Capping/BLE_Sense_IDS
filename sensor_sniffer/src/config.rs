use std::{
    sync::OnceLock,
    io::{BufRead, BufReader, Read, Result},
    fs::File,
    process::Command,
    collections::HashMap,
};
use apache_avro::Schema;
#[allow(unused_imports)]
use serde::Serialize; // for derive

pub const CONFIG_PATH: &str = "./config/config.ini";
pub const AVRO_SCHEMA_PATH: &str = "./config/schema.avsc";
pub const OUI_LOOKUP_PATH: &str = "./config/oui.txt";
pub static SERIAL_ID: OnceLock<u32> = OnceLock::new();
pub static PACKET_BUFFER_SIZE: OnceLock<i32> = OnceLock::new();
pub static PACKET_API_ENDPOINT: OnceLock<String> = OnceLock::new();
pub static HB_API_ENDPOINT: OnceLock<String> = OnceLock::new();
pub static HEARTBEAT_FREQ: OnceLock<u64> = OnceLock::new();
pub static LOGGING: OnceLock<bool> = OnceLock::new();
pub static PCAPNG: OnceLock<bool> = OnceLock::new();
pub static AVRO_SCHEMA: OnceLock<Schema> = OnceLock::new();
pub static OUI_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();
pub static INTERFACE: OnceLock<String> = OnceLock::new();

const LOG: &str = "CONFIG::LOG:";

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
    pub short_device_name: String,     // Device's shortened name
    pub uuids: String                  // A string list of the device's service profiles 
}

// detect the COM port/interface the sniffer connected to
fn get_interface() -> String {
    let output = Command::new("nrfutil")
        .args(&["device", "list"])
        .output()
        .expect("Failed to run nrfutil device list");

    let output_str = String::from_utf8_lossy(&output.stdout);

    // Find the line that starts with "ports"
    for line in output_str.lines() {
        if line.starts_with("ports") {
            // Extract port info (ex. COM3) from the line
            let parts: Vec<&str> = line.split_whitespace().collect();
            return parts[1].to_string(); // port num is the second part (ex. ports    COM3)
        }
    }
    panic!("No valid interface found. Make sure the sniffer is plugged in & receiving ample power.");
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
pub fn load_config() {
    // Load the INI file by creating a map of the contents and setting our globals
    let map: HashMap<String, HashMap<String, Option<String>>> = ini!(CONFIG_PATH);
    // println!("{:#?}", map);

    SERIAL_ID
        .set(map["settings"]["serial_id"].clone().unwrap().parse::<u32>().unwrap())
        .unwrap();

    PACKET_BUFFER_SIZE
        .set(map["settings"]["packet_buffer_size"].clone().unwrap().parse::<i32>().unwrap())
        .unwrap();

    PACKET_API_ENDPOINT
        .set(map["settings"]["packet_api_endpoint"].clone().unwrap())
        .unwrap();

    HB_API_ENDPOINT
        .set(map["settings"]["heartbeat_api_endpoint"].clone().unwrap())
        .unwrap();

    HEARTBEAT_FREQ
        .set(map["settings"]["heartbeat_freq"].clone().unwrap().parse::<u64>().unwrap())
        .unwrap();

    LOGGING
        .set(map["settings"]["logging"].clone().unwrap().to_lowercase().as_str().parse::<bool>().unwrap())
        .unwrap();

    PCAPNG
        .set(map["settings"]["pcapng"].clone().unwrap().to_lowercase().as_str().parse::<bool>().unwrap())
        .unwrap();
    println!("\n{} INI Settings Imported...\n", LOG);

    // load the avro schema into a schema obj for serialization
    let mut schema_file: File = File::open(AVRO_SCHEMA_PATH).expect("Unable to open avro schema file");
    let mut schema_str: String = String::new();
    schema_file.read_to_string(&mut schema_str).expect("Unable to read schema file");
    let schema: Schema = Schema::parse_str(&schema_str).expect("Unable to parse avro schema");

    AVRO_SCHEMA
        .set(schema)
        .unwrap();
    println!("\n{} Avro Schema Loaded...\n", LOG);

    // load the OUI map so we can provide that information
    if OUI_MAP.set(parse_oui_file(OUI_LOOKUP_PATH).unwrap()).is_err() {
        eprintln!("Failed to initialize OUI map");
    } else {
        println!("\n{} OUI Lookup Parsed...\n", LOG);
    }

    // auto detects what port the sniffer identified itself as
    INTERFACE
        .set(get_interface())
        .unwrap();
    println!("\n{} NRF Dongle detected on port: {}\n", LOG, INTERFACE.get().unwrap());
}