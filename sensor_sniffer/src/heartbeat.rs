use std::{
    collections::VecDeque,
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Mutex},
    time::Duration,
    thread,
};
use sysinfo::{
    Disks, Networks, System,
};
use crate::api::send_heartbeat;
use crate::config;

const LOG: &str = "HB::LOG:";

#[derive(Debug)]
#[allow(dead_code)] // because we likely will never read the fields - just send it to API
pub struct SystemInfo {
    total_memory: f32,                     // Total memory in GB
    used_memory: f32,                      // Used memory in GB
    total_swap: f32,                       // Total swap memory in GB
    used_swap: f32,                        // Used swap memory in GB
    total_cpu_usage: f32,                  // Total CPU usage as percentage
    disk_info: Vec<String>,                // List of disk info strings
    network_info: Vec<(String, u64, u64)>, // (Interface name, Total received, Total transmitted)
    packet_queue_length: i32,              // Length of packet queue
}

// Function to gather system information
fn gather_system_info(sys: &mut System, packet_queue_length: i32) -> SystemInfo {
    // Refresh specific components (more efficient than refresh_all)
    sys.refresh_memory();
    sys.refresh_cpu_all();
    
    // Gather memory and swap info in GB (1 GB = 1024^3 bytes or 1,073,741,824 bytes)
    let total_memory: f32 = sys.total_memory() as f32 / 1_073_741_824.0;
    let used_memory: f32 = sys.used_memory() as f32 / 1_073_741_824.0;
    let total_swap: f32 = sys.total_swap() as f32 / 1_073_741_824.0;
    let used_swap: f32 = sys.used_swap() as f32 / 1_073_741_824.0;

    // Gather CPU usage
    let total_cpu_usage: f32 = sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage()) // Sum CPU usage for each core
        .sum::<f32>() / sys.cpus().len() as f32; // Calculate average

    // Gather disk info
    let disks: Disks = Disks::new_with_refreshed_list();
    let disk_info = disks
        .iter()
        .map(|disk| format!("{:?}", disk)) // Convert disk information to a string
        .collect();

    // Gather network info
    let networks: Networks = Networks::new_with_refreshed_list();
    let network_info = networks
        .iter()
        .map(|(interface_name, data)| {
            (
                interface_name.clone(),
                data.total_received(),
                data.total_transmitted(),
            )
        })
        .collect();

    SystemInfo {
        total_memory,
        used_memory,
        total_swap,
        used_swap,
        total_cpu_usage,
        disk_info,
        network_info,
        packet_queue_length,
    }
}

pub fn heartbeat(running: Arc<AtomicBool>, packet_queue: Arc<Mutex<VecDeque<Vec<u8>>>>, sys: &mut System) {
    while running.load(Ordering::SeqCst) {
        let queue_len: i32 = packet_queue.lock().unwrap().len() as i32;
        // println!("Queue length: {}"queue_len);
        let system_info: SystemInfo = gather_system_info(sys, queue_len);

        // Print the system information for debugging (or you can use it in other ways)
        println!("{} System Information: {:#?}", LOG, system_info);
        send_heartbeat(system_info);

        thread::sleep(Duration::from_secs(*config::HEARTBEAT_FREQ.get().unwrap()));
    }
}

