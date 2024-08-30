use std::process::Command;
use std::thread;
use std::time::Duration;

const OUTPUT_DIR: &str = "Sniffed_Data/";  // Change this to your desired directory
const MAX_SIZE_MB: u64 = 1;  // Maximum size of pcapng file before we send it 
const FILE_ROTATION_SIZE: i32 = 10; // Maxiumum number ring buffer files, when 11th file would be created, the oldest file is overwritten.
const SNIFF_INTERFACE: &str = "Wi-Fi"; // interface for tshark to sniff on... WiFi for testing for now

fn start_sniff() {
    // start tshark with ring buffer settings
    Command::new("tshark")
        .args(&[
            "-i", SNIFF_INTERFACE,
            "-b", &format!("filesize:{}", MAX_SIZE_MB * 1024),
            "-b", &format!("files:{}", FILE_ROTATION_SIZE),
            "-w", &format!("{}/capture_ringbuffer.pcapng", OUTPUT_DIR)
        ])
        .spawn()
        .expect("Failed to start sniffing with tshark.");
}

fn monitor_files(){
    loop {
        thread::sleep(Duration::from_secs(5)); // do stuff here
    }
}

fn main() {
    println!("Starting Sniffing!");
    // spawn a separate thread that sniffs data
    thread::spawn(|| {
        start_sniff();
    });

    // monitor the data dir and offload data
    monitor_files();

    println!("Exiting...")
}
