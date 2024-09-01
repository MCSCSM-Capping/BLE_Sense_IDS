use std::process::{Command, Child};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use ctrlc;

const OUTPUT_DIR: &str = "Sniffed_Data/";  // Change this to your desired directory
const MAX_SIZE_MB: u64 = 1;  // Maximum size of pcapng file before we send it 
const FILE_ROTATION_SIZE: i32 = 10; // Maxiumum number ring buffer files, when 11th file would be created, the oldest file is overwritten.
const SNIFF_INTERFACE: &str = "Ethernet"; // interface for tshark to sniff on... eth for testing for now

fn start_sniff() -> Child {
    // start tshark with ring buffer settings
    let tshark = Command::new("tshark")
        .args(&[
            "-i", SNIFF_INTERFACE,
            "-b", &format!("filesize:{}", MAX_SIZE_MB * 1024),
            "-b", &format!("files:{}", FILE_ROTATION_SIZE),
            "-w", &format!("{}/capture_ringbuffer.pcapng", OUTPUT_DIR)
        ])
        .spawn()
        .expect("Failed to start sniffing with tshark.");
    println!("tshark started with PID: {}", tshark.id());

    tshark
}

fn monitor_files(running: Arc<AtomicBool>){
    while running.load(Ordering::SeqCst) {
        // check for files, manage api here
        thread::sleep(Duration::from_secs(5));
    }
}

fn stop_sniff(tshark_process: &mut Child) {
    // kill the tshark sniffing operation
    match tshark_process.kill() {
        Ok(_) => println!("tshark sniffing process killed successfully."),
        Err(err) => eprintln!("Failed to kill tshark process: {}", err),
    }

    // wait for process to fully exit
    match tshark_process.wait() {
        Ok(status) => println!("tshark exited with status: {}", status),
        Err(err) => eprintln!("Error waiting for tshark to exit: {}", err),
    }
}

fn main() {
    println!("Starting Sniffing!");
    // atomic boolean to track if the program should stop
    let running = Arc::new(AtomicBool::new(true));

    // ctrl c handler
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl+C Interrupt Received, shutting down...");
            r.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }
   
    // start sniffer
    let mut tshark_process = start_sniff();
    // monitor the data dir and offload data
    monitor_files(running.clone());

    // Clean shutdown
    println!("Stopping tshark...");
    stop_sniff(&mut tshark_process);
    println!("Exiting Cleanly...");
}
