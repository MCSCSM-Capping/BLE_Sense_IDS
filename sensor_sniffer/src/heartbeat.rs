use std::{
    collections::VecDeque,
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Mutex},
    time::Duration,
    thread,
};
use crate::api;

const LOG: &str = "HB::LOG:";

pub fn heartbeat(running: Arc<AtomicBool>, packet_queue: Arc<Mutex<VecDeque<Vec<u8>>>>, heartbeat_freq: u64) {
    while running.load(Ordering::SeqCst) {
        let queue_len: usize = packet_queue.lock().unwrap().len();
        println!("{} Queue length: {}", LOG, queue_len);
        let message: &str = "PLACEHOLDER";
        api::send_heartbeat(message);
        thread::sleep(Duration::from_secs(heartbeat_freq));
    }
}