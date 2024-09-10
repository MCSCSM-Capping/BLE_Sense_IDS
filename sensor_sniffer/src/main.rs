use std::fs;
use std::path::Path;
use std::process::{Command, Child};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use reqwest::blocking::Client;

const CAPTURE_FILE: &str = "capture.pcapng";
const OUTPUT_DIR: &str = "Sniffed_Data";
const MAX_SIZE: u64 = 500 * 1024; // 500KB
const MAX_FILE_RETENTION: usize = 10;

// Starts the NRFutil ble-sniffer tool for capturing BLE packets
fn start_nrf_sniffer(interface: &String) -> Child {
    let sniffer = Command::new("nrfutil")
        .args(&[
            "ble-sniffer", "sniff", 
            "--port", interface, 
            "--output-pcap-file", &format!("{}/capture.pcapng", OUTPUT_DIR)
        ])
        .spawn()
        .expect("Failed to start nrfutil.");

    println!("nrfSniffer started with PID: {}", sniffer.id());
    sniffer
}

// Checks the size of the file
fn file_size(path: &Path) -> u64 {
    fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

fn offload_file(path: &Path) {
    println!("Send to API!");
    let client = Client::new();
    let api_url = "http://server/api";
    let file_name = path.file_name().unwrap().to_str().unwrap();
    println!("Sending file: {}", file_name);

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

// Rotates the capture file by renaming it
fn rotate_file() -> String {
    let capturefile = format!("{dir}/{file}", dir=OUTPUT_DIR, file=CAPTURE_FILE);
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let new_file = format!("{}/retained_capture_{}.pcapng", OUTPUT_DIR, timestamp);
    fs::rename(capturefile, &new_file).expect("Failed to rename file");

    println!("Rotated file: {}", new_file);
    let new_path = Path::new(&new_file);
    offload_file(new_path);
    new_file
}

// Deletes the oldest files if more than set amount are present
fn delete_oldest_files() {
    let mut files: Vec<_> = fs::read_dir(".")
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_string_lossy().starts_with("capture_"))
        .collect();

    files.sort_by_key(|entry| entry.metadata().unwrap().modified().unwrap());

    while files.len() > MAX_FILE_RETENTION {
        if let Some(old_file) = files.pop() {
            fs::remove_file(old_file.path()).expect("Failed to delete file");
        }
    }
}

fn capture_monitor_offload(running: Arc<AtomicBool>, interface: &String) {
    let path_string = format!("{dir}/{file}", dir=OUTPUT_DIR, file=CAPTURE_FILE);
    let capture_file_path: &Path = Path::new(&path_string);
    // start sniffer
    let mut sniffer: Child = start_nrf_sniffer(interface);

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(5)); // no need to constantly check
        println!("Cap file size: {}", file_size(capture_file_path));
        if file_size(capture_file_path) >= MAX_SIZE {
            // Rotate file out -- simulating a ringbuffer as best as we can
            let __rotated_file = rotate_file();

            // Ensure no more than a specified amount of files persist
            delete_oldest_files();

            // Restart nrfutil to capture in a new file
            stop_sniff(&mut sniffer);
            sniffer = start_nrf_sniffer(interface);
        }
    }  

    // Clean shutdown
    println!("Stopping sniffer...");
    stop_sniff(&mut sniffer);
    println!("Exiting Cleanly...");

}

fn stop_sniff(sniffer: &mut Child) {
    // kill the sniffing operation
    match sniffer.kill() {
        Ok(_) => println!("nrfutil ble-sniffer sniffing process killed successfully."),
        Err(err) => eprintln!("Failed to kill nrf ble-sniffer process: {}", err),
    }

    // wait for process to fully exit
    match sniffer.wait() {
        Ok(status) => println!("nrf sniffer exited with status: {}", status),
        Err(err) => eprintln!("Error waiting for nrf ble-sniffer to exit: {}", err),
    }
}

fn get_interface() -> String {
    let output = Command::new("nrfutil")
        .args(&["device", "list"])
        .output()
        .expect("Failed to run nrfutil device list");

    let output_str = String::from_utf8_lossy(&output.stdout);

    // Find the line that starts with "ports"
    for line in output_str.lines() {
        if line.starts_with("ports") {
            // Extract COM3 or similar from the line
            let parts: Vec<&str> = line.split_whitespace().collect();
            return parts[1].to_string(); // COM3 is the second part
        }
    }
    
    panic!("No valid interface found.");
}

fn clear_artifacts() -> std::io::Result<()> {
    let sniffed_data_dir = Path::new(OUTPUT_DIR);
    if !sniffed_data_dir.exists() {
        fs::create_dir(sniffed_data_dir).expect("Failed to create Sniffed_Data directory");
    }

    let paths = fs::read_dir(OUTPUT_DIR)?;
    let extension = "pcapng";

    for path in paths {
        let path = path?.path();

        // Check if it file and has the desired extension
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some(extension) {
            fs::remove_file(&path)?;
            println!("\tDeleted artifact file: {:?}", path);
        }
    }

    Ok(())
}
fn main() {
    println!("\nStarting sensor!\n");
    println!("Clearing old artifacts...");
    if let Err(e) = clear_artifacts() {
        eprintln!("Failed to clear artifact files: {}", e);
    }

    // auto detects what port the sniffer identified itself as
    let interface: String = get_interface();
    println!("\nDongle detected on port: {}\n", interface);

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
   
    // capture & monitor the data dir, offloading data
    capture_monitor_offload(running.clone(), &interface);

    println!("\nSensor shutting down.\n");
}
