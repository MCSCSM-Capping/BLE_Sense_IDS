use std::{
    collections::VecDeque, sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use apache_avro::Writer;
use tungstenite::Message;
use tokio::{time::sleep, sync::Mutex};
use log::{info, warn};
use crate::config::{
    BLEPacket, PACKET_AVRO_SCHEMA, HB_AVRO_SCHEMA, BACKEND_SOCKET, BACKEND_WEBSOCKET_ENDPOINT,
    OFFLINE, PACKET_BUFFER_SIZE, SERIAL_ID
};
use crate::heartbeat::HeartbeatMessage;

const MAX_RETRIES: u64 = 3;
const RETRY_DELAY_MS: u64 = 500;
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

async fn send_with_retry(data: Vec<u8>, delivery_type: &str) -> bool {
    let mut retries: u64 = 0;

    loop {
        match try_send_message(data.clone()).await {
            Ok(_) => {
                return true;
            }
            Err(e) => {
                retries += 1;
                if retries >= MAX_RETRIES {
                    warn!("Failed to send {} after error: {} retries attempted: {}", delivery_type, MAX_RETRIES, e);
                    return false;
                }
                
                // wait a little longer each time
                sleep(Duration::from_millis(RETRY_DELAY_MS * retries)).await;
            }
        }
    }
}

async fn try_send_message(data: Vec<u8>) -> Result<(), String> {
    let mut socket = BACKEND_SOCKET
        .get()
        .ok_or("WebSocket not initialized!")?
        .lock()
        .map_err(|_| "Failed to lock the WebSocket!")?;

    // Send the encoded message
    socket
        .send(Message::Binary(data))
        .map_err(|e| format!("Failed to send binary message: {}", e))?;

    Ok(())
}

pub async fn deliver_packets(queue: Arc<Mutex<VecDeque<BLEPacket>>>) {
    let mut data_to_send: Vec<BLEPacket> = Vec::new();
    
    // Acquire lock and pop items
    let mut queue_lock = queue.lock().await;
    for _ in 0..*PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
        if let Some(item) = queue_lock.pop_front() {
            data_to_send.push(item);
        }
    }

    // Drop the lock before proceeding
    drop(queue_lock);

    if !*OFFLINE.get().unwrap() {
        let delivery: PacketDelivery = wrap_packet_delivery(data_to_send);
        let encoded_delivery:Vec<u8> = encode_to_packet_avro(delivery);

        let delivery_success: bool = send_with_retry(encoded_delivery, "packet delivery").await;
        if delivery_success {
            info!("Offloaded {} items from queue to endpoint {}.", 
                PACKET_BUFFER_SIZE.get().unwrap(), 
                *BACKEND_WEBSOCKET_ENDPOINT.get().unwrap()
            );
        }
    }
}

pub async fn send_heartbeat(hb_msg: HeartbeatMessage) {
    if !*OFFLINE.get().unwrap() {
        let encoded_msg: Vec<u8> = encode_to_hb_avro(hb_msg);
        
        let delivery_sucess: bool = send_with_retry(encoded_msg, "heartbeat").await;
        if delivery_sucess {
            info!("Sent Heartbeat Message to endpoint: {}.",  
                *BACKEND_WEBSOCKET_ENDPOINT.get().unwrap()
            );
        }
    }
}