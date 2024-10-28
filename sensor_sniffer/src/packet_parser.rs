use std::collections::HashMap;
use regex::Regex;
use crate::config::{BLEPacket, OUI_MAP};

// take in a string of hex values that is the advertising payload of a BLE packet and parse it to get attributes from it
fn parse_advertising_data(advertising_data_hex: &str) -> HashMap<String, String> {
    let advertising_data: Vec<u8> = hex::decode(advertising_data_hex)
        .expect("Failed to decode advertising data hex string");

    let mut index: usize = 0;
    let mut result: HashMap<String, String> = HashMap::new();
    let mut uuid_list: Vec<String> = Vec::new();

    // Insert default values in case info is not provided in the adv data (common)
    result.insert("long_device_name".to_string(), "Unknown".to_string());
    result.insert("short_device_name".to_string(), "Unknown".to_string());
    result.insert("company_id".to_string(), "-1".to_string());
    result.insert("power_level".to_string(), "-255".to_string());

    // check the values for 'indicators' that describe the data to follow
    while index < advertising_data.len() {
        let length: usize = advertising_data[index] as usize;
        if length == 0 {
            break;
        }

        let ad_type: u8 = advertising_data[index + 1];
        let ad_data: &[u8] = &advertising_data[index + 2..index + length + 1];

        // keep note that little endian is used
        match ad_type {
            0x01 => {
                // Flags - just ignore for now.
            }
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
            0x02 | 0x03 => {
                // Incomplete or Complete List of 16-bit UUIDs
                for chunk in ad_data.chunks_exact(2) {
                    if let [b1, b2] = chunk {
                        let uuid: String = format!("{:04X}", (*b2 as u16) << 8 | (*b1 as u16));
                        uuid_list.push(uuid);
                    }
                }
            }
            0x06 | 0x07 => {
                // Incomplete or Complete List of 128-bit UUIDs
                for chunk in ad_data.chunks_exact(16) {
                    let uuid: String = chunk.iter().rev().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join("");
                    uuid_list.push(uuid);
                }
            }
            _ => {
                // Other data types can be handled here
            }
        }

        index += length + 1;
    }

    if uuid_list.len() == 0{
        result.insert("uuids".to_string(), "None".to_string());
    } else {
        result.insert("uuids".to_string(), uuid_list.join(", "));
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
pub fn parse_ble_packet(input: &str) -> BLEPacket {
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
        .unwrap_or(-1.0); // Default to 0.0 if parsing fails

    let rssi: i32 = rssi_re
        .captures(input)
        .and_then(|cap: regex::Captures<'_>| cap.get(1).map(|m: regex::Match<'_>| m.as_str().parse::<i32>().ok()))
        .flatten()
        .unwrap_or(1); // Default to 1 if parsing fails - rssi caps at 0

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
    let mut power_level: i32 = -255; // so default value is out of range 
    let mut company_id: i32 = -1;
    let mut long_device_name: String = "Unknown".to_string();
    let mut short_device_name: String = "Unknown".to_string();
    let mut uuids: String = "None".to_string();
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
        uuids = parsed_adv_data["uuids"].to_string();
    } 

    let mut oui: String = "Unknown".to_string();
    if advertising_address != -1 {
        oui = lookup_oui(advertising_address);
    } 

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
        short_device_name,
        uuids
    }
}