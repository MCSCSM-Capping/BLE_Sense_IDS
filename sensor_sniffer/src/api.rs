use std::{
    collections::VecDeque, sync::{Arc, Mutex},
};
use apache_avro::Writer;
use tungstenite::{Message, WebSocket};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::config::{
    PACKET_BUFFER_SIZE, AVRO_SCHEMA, SERIAL_ID, LOGGING, OFFLINE,
    PACKET_API_ENDPOINT, HB_API_ENDPOINT, BLEPacket, BACKEND_SOCKET,
};
use crate::heartbeat::HeartbeatMessage;

const LOG: &str = "API::LOG:";

// use the schema to encode/serialize a BLEPacket to avro
pub fn encode_avro(packet: BLEPacket) -> Vec<u8> {
    let mut writer: Writer<'_, Vec<u8>> = Writer::new(&AVRO_SCHEMA.get().unwrap(), Vec::new());
    writer.append_ser(packet).expect("Unable to serialize data");
    let encoded_data: Vec<u8> = writer.into_inner().expect("Unable to get encoded data");

    encoded_data
}

// release encoded packets to the api
pub fn offload_to_api(queue: Arc<Mutex<VecDeque<Vec<u8>>>>) {
    // create object to offload via API - its the first PACKET_BUFFER_SIZE packets of the queue
    let mut data_to_send: Vec<Vec<u8>> = Vec::new();
    for _ in 0..*PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
        if let Some(item) = queue.lock().unwrap().pop_front() {
            data_to_send.push(item);
        }
    }

    if !*OFFLINE.get().unwrap() {
        let mut __socket = BACKEND_SOCKET
            .get()
            .expect("WebSocket not initialized.")
            .lock()
            .expect("Failed to lock the WebSocket.");

        // sending as octet? 
    }

    println!("{} Offloaded {} items from queue to endpoint {}.", LOG, PACKET_BUFFER_SIZE.get().unwrap(), *PACKET_API_ENDPOINT.get().unwrap());
}

// deliver HB message
pub fn send_heartbeat(hb_msg: HeartbeatMessage) {
    if !*OFFLINE.get().unwrap() {
        let mut socket = BACKEND_SOCKET
            .get()
            .expect("WebSocket not initialized.")
            .lock()
            .expect("Failed to lock the WebSocket.");

        let json_msg: String = serde_json::to_string(&hb_msg).expect("Failed to serialize object");
        socket
            .send(Message::Text(json_msg))
            .expect("Failed to send heartbeat message.");

        if *LOGGING.get().unwrap() {
            println!("{} Sent Heartbeat Message to endpoint: {}.", LOG, *HB_API_ENDPOINT.get().unwrap());
        }
    }

}