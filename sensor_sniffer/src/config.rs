use std::{
    sync::{Mutex, OnceLock},
    io::{BufRead, BufReader, Read, Result},
    fs::File,
    process::Command,
    collections::HashMap,
};
use apache_avro::Schema;
#[allow(unused_imports)]
use serde::Serialize; // for derive
use tungstenite::{connect, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;
use log::{info, trace, error};

pub const CONFIG_PATH: &str = "./config/config.ini";
pub const PACKET_AVRO_SCHEMA_PATH: &str = "./config/packet_schema.avsc";
pub const HB_AVRO_SCHEMA_PATH: &str = "./config/hb_schema.avsc";
pub const OUI_LOOKUP_PATH: &str = "./config/oui.txt";
pub static SERIAL_ID: OnceLock<u32> = OnceLock::new();
pub static PACKET_BUFFER_SIZE: OnceLock<i32> = OnceLock::new();
pub static BACKEND_WEBSOCKET_ENDPOINT: OnceLock<String> = OnceLock::new();
pub static HEARTBEAT_FREQ: OnceLock<u64> = OnceLock::new();
pub static PCAPNG: OnceLock<bool> = OnceLock::new();
pub static PACKET_AVRO_SCHEMA: OnceLock<Schema> = OnceLock::new();
pub static HB_AVRO_SCHEMA: OnceLock<Schema> = OnceLock::new();
pub static OUI_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();
pub static INTERFACE: OnceLock<String> = OnceLock::new();
pub static TEST_MODE: OnceLock<bool> = OnceLock::new();
pub static BACKEND_SOCKET: OnceLock<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>> = OnceLock::new();
pub static OFFLINE: OnceLock<bool> = OnceLock::new();

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
    // trace!("{:#?}", map);

    SERIAL_ID
        .set(map["settings"]["serial_id"].clone().unwrap().parse::<u32>().unwrap())
        .unwrap();

    PACKET_BUFFER_SIZE
        .set(map["settings"]["packet_buffer_size"].clone().unwrap().parse::<i32>().unwrap())
        .unwrap();

    BACKEND_WEBSOCKET_ENDPOINT
        .set(map["settings"]["backend_websocket_endpoint"].clone().unwrap())
        .unwrap();

    HEARTBEAT_FREQ
        .set(map["settings"]["heartbeat_freq"].clone().unwrap().parse::<u64>().unwrap())
        .unwrap();

    PCAPNG
        .set(map["settings"]["pcapng"].clone().unwrap().to_lowercase().as_str().parse::<bool>().unwrap())
        .unwrap();
   
    TEST_MODE
        .set(map["settings"]["test_mode"].clone().unwrap().to_lowercase().as_str().parse::<bool>().unwrap())
        .unwrap();

    OFFLINE
        .set(map["settings"]["offline"].clone().unwrap().to_lowercase().as_str().parse::<bool>().unwrap())
        .unwrap();

    trace!("INI Settings Imported...");

    // load the avro schema into a schema obj for serialization
    let mut packet_schema_file: File = File::open(PACKET_AVRO_SCHEMA_PATH).expect("Unable to open packet avro schema file");
    let mut packet_schema_str: String = String::new();
    packet_schema_file.read_to_string(&mut packet_schema_str).expect("Unable to read packet schema file");
    let packet_schema: Schema = Schema::parse_str(&packet_schema_str).expect("Unable to parse packet avro schema");
    PACKET_AVRO_SCHEMA
        .set(packet_schema)
        .unwrap();
    trace!("Packet Avro Schema Loaded...");

    let mut hb_schema_file: File = File::open(HB_AVRO_SCHEMA_PATH).expect("Unable to open heartbeat avro schema file");
    let mut hb_schema_str: String = String::new();
    hb_schema_file.read_to_string(&mut hb_schema_str).expect("Unable to read heartbeat schema file");
    let hb_schema: Schema = Schema::parse_str(&hb_schema_str).expect("Unable to parse heartbeat avro schema");
    HB_AVRO_SCHEMA
        .set(hb_schema)
        .unwrap();
    trace!("Heartbeat Avro Schema Loaded...");

    // load the OUI map so we can provide that information
    if OUI_MAP.set(parse_oui_file(OUI_LOOKUP_PATH).unwrap()).is_err() {
        error!("Failed to initialize OUI map");
    } else {
        trace!("OUI Lookup Parsed...");
    }

    if !*OFFLINE.get().unwrap() {
        let (socket, response) = 
            connect(&*BACKEND_WEBSOCKET_ENDPOINT.get().unwrap().as_str()).expect("Web Socket Conenction FAILED.");
        info!("Connected to the websocket {:?}", response);
        BACKEND_SOCKET
            .set(Mutex::new(socket))
            .expect("Failure to save websocket connection.");
    }

    // we don't use the sniffer in test mode
    if !*TEST_MODE.get().unwrap() {
        // auto detects what port the sniffer identified itself as
        INTERFACE
            .set(get_interface())
            .unwrap();
        info!("NRF Dongle detected on port: {}", INTERFACE.get().unwrap());
    }
}