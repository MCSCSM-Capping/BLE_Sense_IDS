use std::process::{Command, Child, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{BufReader, BufRead};
use std::sync::Arc;
use reqwest::blocking::Client;
use std::collections::VecDeque;

const QUEUE_MAX_SIZE: usize = 500;
const API_ENDPOINT: &str = "http://server/api";

// Starts the NRFutil ble-sniffer tool for capturing BLE packets
fn start_nrf_sniffer(interface: &String) -> Child {
    let sniffer = Command::new("nrfutil")
        .args(&[
            "ble-sniffer", "sniff", // call sniffer
            "--port", interface,    // sniff on interface we detected it on
            "--log-output=stdout",  // send logs to stdout
            "--json",               // so output is formatted
            "--log-level", "debug", // so we can see packets in stdout
            "--output-pcap-file", "NUL" // trick nrf into running but not creating its pcapng
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start nrfutil.");
    println!("nrfSniffer started with PID: {}", sniffer.id());
    sniffer
}

fn offload(queue: &mut VecDeque<String>) {
    println!("Send to API!");
    let __client = Client::new();

    // create object to offload via API
    let mut data_to_send = Vec::new();
    for _ in 0..QUEUE_MAX_SIZE {
        if let Some(item) = queue.pop_front() {
            data_to_send.push(item);
        }
    }

    // let response = client.post(api_url)
    //     .body(buffer)
    //     .header("Content-Type", "application/octet-stream")
    //     .send();

    // match response {
    //     Ok(resp) => println!("File sent successfully: {}, Response: {:?}", file_name, resp),
    //     Err(err) => eprintln!("Failed to send file: {}, Error: {}", file_name, err),
    // }
}

fn parse_offload(running: Arc<AtomicBool>, interface: &String) {
    let mut queue: VecDeque<String> = VecDeque::new(); // queue to hold data
    // start sniffer
    let mut sniffer: Child = start_nrf_sniffer(interface);

    while running.load(Ordering::SeqCst) {
        if let Some(stdout) = sniffer.stdout.take() {
            let reader = BufReader::new(stdout);
    
            // Read the stdout line by line as it comes in
            for line in reader.lines() {
                let line = line.expect("Could not read line from stdout");
                //println!("{}", line.clone());
                queue.push_back(line);

                if queue.len() == QUEUE_MAX_SIZE {
                    offload(&mut queue);
                }
            }
        }
    }  

    // dump_queue() if shutting down, might want to dump the queue to api first
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
fn main() {
    println!("\nStarting sensor!\n");

    // auto detects what port the sniffer identified itself as
    let interface: String = get_interface();
    println!("\nNRF Dongle detected on port: {}\n", interface);

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
    parse_offload(running.clone(), &interface);

    println!("\nSensor shut down.\n");
}
