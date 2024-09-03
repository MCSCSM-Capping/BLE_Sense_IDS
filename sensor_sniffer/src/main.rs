use std::process::{Command, Child};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::collections::VecDeque;
use reqwest::blocking::Client;
use std::time::Duration;
use ctrlc;

const OUTPUT_DIR: &str = "Sniffed_Data";  // Change this to your desired directory
const MAX_SIZE_KB: u64 = 200;  // Maximum size of pcapng file before we send it 
const FILE_ROTATION_SIZE: usize = 5; // Maximum number ring buffer files, when 6th file would be created, the oldest file is overwritten.
const SNIFF_INTERFACE: &str = "Com3-4.4"; // interface for tshark to sniff on... eth for testing for now
const KERNEL_BUFFER_SIZE_MB: u32 = 10; // size of kernel buffer for tshark to temp store data
const SNAPLEN: u32 = 60;  // Capture the first 60 bytes of each BLE packet aka the headers
const UPDATE_INTERVAL_MS: u32 = 10;  // Set the update interval to 50 ms for time between packets

fn start_sniff() -> Child {
    // start tshark with ring buffer settings
    let sniffed_data_dir = Path::new(OUTPUT_DIR);
    if !sniffed_data_dir.exists() {
        fs::create_dir(sniffed_data_dir).expect("Failed to create Sniffed_Data directory");
    }
    
    let tshark = Command::new("tshark")
        .args(&[
            "-i", SNIFF_INTERFACE,
            "-b", &format!("filesize:{}", MAX_SIZE_KB),
            "-b", &format!("files:{}", FILE_ROTATION_SIZE),
            "-B", &format!("{}", KERNEL_BUFFER_SIZE_MB),
            "-s", &format!("{}", SNAPLEN),
            "--update-interval", &format!("{}", UPDATE_INTERVAL_MS),
            "-w", &format!("{}/capture_ringbuffer.pcapng", OUTPUT_DIR)
        ])
        .spawn()
        .expect("Failed to start sniffing with tshark.");
    println!("tshark started with PID: {}", tshark.id());

    tshark
}

fn is_pcapng_file(path: &Path) -> bool {
    path.extension().map_or(false, |ext| ext == "pcapng")
}

fn reached_max_size(path: &Path) -> bool {
    match fs::metadata(path) {
        Ok(metadata) => {
            let file_size = metadata.len();
            println!("File: {:?}, Size: {} Kbytes", path.file_name().unwrap(), file_size/1024);
            (file_size/1024) >= (MAX_SIZE_KB - 5) // files don't reach full size, they stop early
        },
        Err(_) => false,
    }
}

fn send_file(client: &Client, api_url: &str, path: &Path) {
    println!("Send to API!");
    // let file_name = path.file_name().unwrap().to_str().unwrap();
    // println!("Sending file: {}", file_name);

    // let mut file = fs::File::open(path).unwrap();
    // let mut buffer = Vec::new();
    // file.read_to_end(&mut buffer).unwrap();

    // let response = client.post(api_url)
    //     .body(buffer)
    //     .header("Content-Type", "application/octet-stream")
    //     .send();

    // match response {
    //     Ok(resp) => println!("File sent successfully: {}, Response: {:?}", file_name, resp),
    //     Err(err) => eprintln!("Failed to send file: {}, Error: {}", file_name, err),
    // }
}

fn monitor_files(running: Arc<AtomicBool>) {
    // queue contains files we already sent to API. When the ring buffer deletes the oldest file as the limit was reached,
    // the queue will also remove the oldest entry as to save memory.
    let mut sent_files: VecDeque<PathBuf> = VecDeque::with_capacity(FILE_ROTATION_SIZE);

    while running.load(Ordering::SeqCst) {
        let client = Client::new();
        let api_url = "http://server/api";  // TODO
        let sniffed_data_dir = Path::new(OUTPUT_DIR);

        // Check for new pcapng in dir
        let files = fs::read_dir(sniffed_data_dir).unwrap();
        for file in files {
            let file = file.unwrap();
            let path = file.path();
           
            let file_name = path.file_name().unwrap().to_str().unwrap();
            println!("Checking file: {}", file_name);
            println!("size?: {}", reached_max_size(&path));
            println!("in queue? {}", sent_files.contains(&path));

            // If the file is a pcapng file and has reached the max size, send it
            if is_pcapng_file(&path) && reached_max_size(&path) && !sent_files.contains(&path) {
                println!("Conditions Met");
                send_file(&client, &api_url, &path);
                
                // Manage the queue
                if sent_files.len() == FILE_ROTATION_SIZE {
                    sent_files.pop_front();
                }

            sent_files.push_back(path.clone());
            }
            
        }

        thread::sleep(Duration::from_secs(1));
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
