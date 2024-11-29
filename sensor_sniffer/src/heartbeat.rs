use std::{
    collections::VecDeque,
    sync::Arc,
    fmt::Write,
    time::{Duration, SystemTime, UNIX_EPOCH}
};
use sysinfo::{
    Disks, Networks, System,
};
use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex, signal};
use log::{trace, info};
use crate::socket::send_heartbeat;
use crate::config::{SERIAL_ID, HEARTBEAT_FREQ, BLEPacket};

const LOG: &str = "HB::LOG:";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HeartbeatMessage {
    serial: u32,
    timestamp: String,
    body: SystemInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)] // because we likely will never read the fields - just send it to API
pub struct SystemInfo {
    total_memory: f32,                     // Total memory in GB
    used_memory: f32,                      // Used memory in GB
    total_swap: f32,                       // Total swap memory in GB
    used_swap: f32,                        // Used swap memory in GB
    total_cpu_usage: f32,                  // Total CPU usage as percentage
    disk_info: Vec<String>,                // List of disk info strings
    network_info: Vec<NetworkInfo>, // (Interface name, Total received, Total transmitted)
    packet_queue_length: i32,              // Length of packet queue
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkInfo {
    interface_name: String,
    total_received: u64,
    total_transmitted: u64,
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

    let total_cpu_usage: f32 = sys.cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage()) // Sum CPU usage for each core
        .sum::<f32>() / sys.cpus().len() as f32; // Calculate average

    let disks: Disks = Disks::new_with_refreshed_list();
    let disk_info = disks
        .iter()
        .map(|disk| format!("{:?}", disk)) // Convert disk information to a string
        .collect();

        let networks: Networks = Networks::new_with_refreshed_list();
        let network_info: Vec<NetworkInfo> = networks
            .iter()
            .map(|(interface_name, data)| NetworkInfo {
                interface_name: interface_name.clone(),
                total_received: data.total_received(),
                total_transmitted: data.total_transmitted(),
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

async fn beat_heart(packet_queue: Arc<Mutex<VecDeque<BLEPacket>>>, sys: &mut System) {
    let queue_len: i32 = packet_queue.lock().await.len() as i32;
    let system_info: SystemInfo = gather_system_info(sys, queue_len);

    let time: SystemTime = SystemTime::now();
    let duration: Duration = time.duration_since(UNIX_EPOCH).unwrap();
    let mut datetime: String = String::new();
    write!(&mut datetime, "{}", duration.as_secs()).unwrap();

    let hb_msg: HeartbeatMessage = HeartbeatMessage {
        serial: *SERIAL_ID.get().unwrap(),
        timestamp: datetime,
        body: system_info,
    };

    trace!("{} Heartbeat Message: {:#?}", LOG, hb_msg);

    // Spawn the task to send heartbeat
    tokio::spawn(send_heartbeat(hb_msg));
}

pub async fn heartbeat(packet_queue: Arc<Mutex<VecDeque<BLEPacket>>>) {
    // system obj used to collect load & resource information
    let mut sys: System = System::new_all();
    beat_heart(packet_queue.clone(), &mut sys).await; // inital beat on startup

    loop {
        tokio::select! {
            // Sleep for the heartbeat interval
            _ = tokio::time::sleep(Duration::from_secs(*HEARTBEAT_FREQ.get().unwrap())) => {
                beat_heart(packet_queue.clone(), &mut sys).await;
            }

            // Handle Ctrl+C signal (to stop the heartbeat)
            _ = signal::ctrl_c() => {
                info!("Ctrl+C received by heartbeat.");
                break; // Exit the loop when the interrupt signal is received
            }
        }
    }
    info!("Heartbeat Process Halted.");
}

