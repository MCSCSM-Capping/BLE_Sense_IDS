use rand::{distributions::Alphanumeric, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config::BLEPacket;

pub fn generate_random_packet() -> BLEPacket {
    let mut rng = rand::thread_rng();

    BLEPacket {
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64(),
        rssi: rng.gen_range(-100..0),  // RSSI ranges from -100 to 0 dBm
        channel_index: rng.gen_range(0..40),  // BLE channels: 0-39
        advertising_address: rng.gen_range(1_000_000_000_000..9_999_999_999_999),
        company_id: rng.gen_range(-1..1000),  // -1 for unknown, otherwise a valid ID
        packet_counter: rng.gen_range(1000..3000),
        protocol_version: 3,  
        power_level: rng.gen_range(1..10),  
        oui: random_manufacturer(),
        long_device_name: random_device_name(15),  // Up to 15-character device name
        short_device_name: random_device_name(8),  // Up to 8-character short name
        uuids: random_uuid(),
    }
}

fn random_manufacturer() -> String {
    let manufacturers = vec!["Unknown", "CANON INC.", "Samsung", "Apple"];
    let mut rng = rand::thread_rng();
    manufacturers[rng.gen_range(0..manufacturers.len())].to_string()
}

fn random_device_name(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

fn random_uuid() -> String {
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| format!("{:X}", rng.gen_range(0..16)))
        .collect()
}
