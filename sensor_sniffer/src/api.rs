use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use apache_avro::Writer;
use reqwest::blocking::Client;
use crate::config;

const LOG: &str = "API::LOG:";

// use the schema to encode/serialize a BLEPacket to avro
pub fn encode_avro(packet: config::BLEPacket) -> Vec<u8> {
    let mut writer: Writer<'_, Vec<u8>> = Writer::new(&config::AVRO_SCHEMA.get().unwrap(), Vec::new());
    writer.append_ser(packet).expect("Unable to serialize data");
    let encoded_data: Vec<u8> = writer.into_inner().expect("Unable to get encoded data");

    encoded_data
}

// release encoded packets to the api
pub fn offload_to_api(queue: Arc<Mutex<VecDeque<Vec<u8>>>>) {
    // create object to offload via API - its the first PACKET_BUFFER_SIZE packets of the queue
    let mut data_to_send: Vec<Vec<u8>> = Vec::new();
    for _ in 0..*config::PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
        if let Some(item) = queue.lock().unwrap().pop_front() {
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
    println!("{} Offloaded {} items from queue to endpoint.", LOG, config::PACKET_BUFFER_SIZE.get().unwrap());
}

pub fn send_heartbeat(__message: &str) {
    println!("{} Sent Heartbeat Message.", LOG);
}