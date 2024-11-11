use std::{
    collections::VecDeque, sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use apache_avro::Writer;
use tungstenite::Message;
use crate::config::{
    BLEPacket, PACKET_AVRO_SCHEMA, HB_AVRO_SCHEMA, BACKEND_SOCKET, BACKEND_WEBSOCKET_ENDPOINT, 
    LOGGING, OFFLINE, PACKET_BUFFER_SIZE, SERIAL_ID
};
use crate::heartbeat::HeartbeatMessage;

const LOG: &str = "API::LOG:";
#[derive(Debug, Clone, serde::Serialize)]
pub struct PacketDelivery {
    pub serial_id: u32,
    pub timestamp: u64,
    pub packets: Vec<BLEPacket>,
}

// use the schema to encode/serialize data to avro
fn encode_to_packet_avro(delivery: PacketDelivery) -> Vec<u8> {    
    let mut writer: Writer<'_, Vec<u8>> = Writer::new(&PACKET_AVRO_SCHEMA.get().unwrap(), Vec::new());
    writer.append_ser(delivery).expect("Unable to serialize data");
    let encoded_data: Vec<u8> = writer.into_inner().expect("Unable to get encoded data");

    encoded_data
}

fn encode_to_hb_avro(hb: HeartbeatMessage) -> Vec<u8> {
    let mut writer: Writer<'_, Vec<u8>> = Writer::new(&HB_AVRO_SCHEMA.get().unwrap(), Vec::new());
    writer.append_ser(hb).expect("Unable to serialize data");
    let encoded_data: Vec<u8> = writer.into_inner().expect("Unable to get encoded data");

    encoded_data
}

fn wrap_packet_delivery(packets: Vec<BLEPacket>) -> PacketDelivery {
    let time: SystemTime = SystemTime::now();
    let duration: Duration = time.duration_since(UNIX_EPOCH).unwrap();
    let timestamp: u64 = duration.as_secs();

    let serial_id: u32 = *SERIAL_ID.get().unwrap();

    PacketDelivery {
        timestamp,
        serial_id,
        packets,
    }
}

// release encoded packets to the websocket
pub fn deliver_packets(queue: Arc<Mutex<VecDeque<BLEPacket>>>) {
    // create object to offload via socket - its the first PACKET_BUFFER_SIZE packets of the queue
    let mut data_to_send: Vec<BLEPacket> = Vec::new();
    for _ in 0..*PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
        if let Some(item) = queue.lock().unwrap().pop_front() {
            data_to_send.push(item);
        }
    }

    if !*OFFLINE.get().unwrap() {
        // Lock the Mutex to get mutable access to the WebSocket
        let mut socket = BACKEND_SOCKET
            .get()
            .expect("WebSocket not initialized.")
            .lock()
            .expect("Failed to lock the WebSocket.");

        let delivery: PacketDelivery = wrap_packet_delivery(data_to_send);
        let encoded_delivery: Vec<u8> = encode_to_packet_avro(delivery);

        socket
            .send(Message::Binary(encoded_delivery))
            .expect("Failed to send binary packet delivery!");
    }

    if *LOGGING.get().unwrap() {
        println!("{} Offloaded {} items from queue to endpoint {}.", LOG, PACKET_BUFFER_SIZE.get().unwrap(), *BACKEND_WEBSOCKET_ENDPOINT.get().unwrap());
    }
}

// deliver HB message
pub fn send_heartbeat(hb_msg: HeartbeatMessage) {
    if !*OFFLINE.get().unwrap() {
        let mut socket = BACKEND_SOCKET
            .get()
            .expect("WebSocket not initialized.")
            .lock()
            .expect("Failed to lock the WebSocket.");

        let encoded_msg: Vec<u8> = encode_to_hb_avro(hb_msg);
        socket
            .send(Message::Binary(encoded_msg))
            .expect("Failed to send binary heartbeat message.");

        if *LOGGING.get().unwrap() {
            println!("{} Sent Heartbeat Message to endpoint: {}.", LOG, *BACKEND_WEBSOCKET_ENDPOINT.get().unwrap());
        }
    }
}
