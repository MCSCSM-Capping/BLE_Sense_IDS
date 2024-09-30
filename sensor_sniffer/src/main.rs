use reqwest::blocking::Client;
use std::collections::VecDeque;
use std::sync::OnceLock;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
#[macro_use]
extern crate ini;


const CONFIG_PATH: &str = "./config/config.ini";
static SERIAL_ID: OnceLock<u32> = OnceLock::new();
static PACKET_BUFFER_SIZE: OnceLock<i32> = OnceLock::new();
static API_ENDPOINT: OnceLock<String> = OnceLock::new();
static HEARTBEAT_FREQ: OnceLock<u32> = OnceLock::new();

// loads program constants from INI file
fn load_config() {
    // Load the INI file
    let map = ini!(CONFIG_PATH);
    // println!("{:#?}", map);

    SERIAL_ID
        .set(map["settings"]["serial_id"].clone().unwrap().parse::<u32>().unwrap())
        .unwrap();

    PACKET_BUFFER_SIZE
        .set(map["settings"]["packet_buffer_size"].clone().unwrap().parse::<i32>().unwrap())
        .unwrap();

    API_ENDPOINT
        .set(map["settings"]["api_endpoint"].clone().unwrap())
        .unwrap();

    HEARTBEAT_FREQ
        .set(map["settings"]["heartbeat_freq"].clone().unwrap().parse::<u32>().unwrap())
        .unwrap();
}

// Starts the NRFutil ble-sniffer tool for capturing BLE packets
fn start_nrf_sniffer(interface: &String) -> Child {
    let sniffer = Command::new("nrfutil")
        .args(&[
            "ble-sniffer",
            "sniff", // call sniffer
            "--port",
            interface,             // sniff on interface we detected it on
            "--log-output=stdout", // send logs to stdout
            "--json",              // so output is formatted
            "--log-level",
            "debug", // so we can see packets in stdout
            "--output-pcap-file",
            "NUL", // trick nrf into running but not creating its pcapng
        ])
        .stdout(Stdio::piped()) // pipe stdout so rust can capture and process it
        .spawn() // spawn the process
        .expect("Failed to start nrfutil.");
    println!("nrfSniffer started with PID: {}", sniffer.id());
    return sniffer; // return process so we can reference it later
}

fn offload_to_api(queue: &mut VecDeque<String>) {
    println!("Offloading to API!");

    // create object to offload via API - its the first PACKET_BUFFER_SIZE packets of the queue
    let mut data_to_send = Vec::new();
    for _ in 0..*PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
        if let Some(item) = queue.pop_front() {
            data_to_send.push(item);
        }
    }
    

    // maybe move client creation to global so it isn't made every time this is called
    let __client = Client::new();
    // send the dequeued packets to API
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
    let mut packet_queue: VecDeque<String> = VecDeque::new(); // queue to hold data
                                                              // start sniffer
    let mut sniffer: Child = start_nrf_sniffer(interface);

    // we do this loop forever (or until interrupt)
    while running.load(Ordering::SeqCst) {
        // capture the nrf info from stdout
        if let Some(stdout) = sniffer.stdout.take() {
            let reader = BufReader::new(stdout);
            // Read the stdout line by line as it comes in
            for line in reader.lines() {
                let line = line.expect("Could not read line from stdout");
                //println!("{}", line.clone());
                if line.contains("Parsed packet") {
                    // atm we only want packet data
                    // cut nrf log header and remove trailing brackets
                    let packet: String = line[66..line.len() - 2].to_string();
                    packet_queue.push_back(packet); // add packet to end of queue
                                                    // println!("Queue Size: {}", queue.len());
                }

                if packet_queue.len() >= *PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
                    offload_to_api(&mut packet_queue); // by reference so offload can empty queue FIFO
                }
            }
        }
    }
    // dump_queue() if shutting down, might want to dump the queue to api first?
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
            // Extract port info (ex. COM3) from the line
            let parts: Vec<&str> = line.split_whitespace().collect();
            return parts[1].to_string(); // port num is the second part (ex. ports    COM3)
        }
    }
    panic!("No valid interface found.");
}
fn main() {
    println!("\nLoading Config...\n");
    load_config();

    println!("\nStarting Sensor (Serial: {})!\n", SERIAL_ID.get().unwrap());
    // auto detects what port the sniffer identified itself as
    let interface: String = get_interface();
    println!("\nNRF Dongle detected on port: {}\n", interface);

    // atomic boolean to track if the program should stop
    let running = Arc::new(AtomicBool::new(true));
    // ctrl c handler - so the program will exit the infinite loop
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl+C Interrupt Received, shutting down...");
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    // capture packets and periodically send to api
    parse_offload(running.clone(), &interface);

    println!("\nSensor shut down.\n");
}
