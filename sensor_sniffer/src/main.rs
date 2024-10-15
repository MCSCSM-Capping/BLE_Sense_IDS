mod heartbeat;
mod packet_parser;
mod config;
mod api;
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
    fs::File,
};
use sysinfo::System;
extern crate hex;
#[macro_use]
extern crate ini;

const LOG: &str = "MAIN::LOG:";

// Starts the NRFutil ble-sniffer tool for capturing BLE packets
fn start_nrf_sniffer() -> Child {
    let pcapng_redirect: &str;
    if *config::PCAPNG.get().unwrap() {
        pcapng_redirect = "sensor_capture.pcapng";
    } else if cfg!(target_os = "windows") {
        pcapng_redirect = "NUL"; // Windows
    } else {
        pcapng_redirect = "/dev/null"; // Linux/macOS
    };

    let sniffer: Child = Command::new("nrfutil")
        .args(&[
            "ble-sniffer",
            "sniff", // call sniffer
            "--port",
            &config::INTERFACE.get().unwrap(),  // sniff on interface we detected it on
            "--log-output=stdout",      // send logs to stdout
            "--json",                   // so output is formatted
            "--log-level",
            "debug", // so we can see packets in stdout
            "--output-pcap-file",
            pcapng_redirect, // trick nrf into running but not creating its pcapng
        ])
        .stdout(Stdio::piped()) // pipe stdout so rust can capture and process it
        .spawn() // spawn the process
        .expect("Failed to start nrfutil.");
    println!("{} nrfSniffer started with PID: {}", LOG, sniffer.id());
    return sniffer; // return process so we can reference it later
}

// constantly take in the output of nrfutil. stop with an interrupt. manage api sending.
fn parse_offload(running: Arc<AtomicBool>, packet_queue: Arc<Mutex<VecDeque<Vec<u8>>>>) {
    let mut sniffer: Child = start_nrf_sniffer(); // start sniffer

    // we do this loop forever (or until interrupt)
    while running.load(Ordering::SeqCst) {
        // capture the nrf info from stdout
        if let Some(stdout) = sniffer.stdout.take() {
            let reader = BufReader::new(stdout);
            // Read the stdout line by line as it comes in
            for line in reader.lines() {
                let line: String = line.expect("Could not read line from stdout");
                // println!("{}", line.clone());
                if line.contains("Parsed packet") {
                    let parsed_ble_packet: config::BLEPacket = packet_parser::parse_ble_packet(&line); 
                    let encoded_packet: Vec<u8> = api::encode_avro(parsed_ble_packet.clone());
                    
                    if *config::LOGGING.get().unwrap() {
                        println!("\n\n{}", LOG);
                        println!("{}", line);
                        println!("{:#?}", parsed_ble_packet);
                    }

                    println!("{:?}", encoded_packet);
                    packet_queue.lock().unwrap().push_back(encoded_packet);
                    // println!("Queue Size: {}", queue.len());
                }

                if packet_queue.lock().unwrap().len() >= *config::PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
                    api::offload_to_api(packet_queue.clone()); // by reference so offload can empty queue FIFO
                }
            }
        }
    }
    // dump_queue(); if shutting down, might want to dump the queue to api first to avoid data loss?
}

fn test_simulation(running: Arc<AtomicBool>, packet_queue: Arc<Mutex<VecDeque<Vec<u8>>>>) {
    const DELAY: Duration = Duration::from_millis(100);
    const TEST_DATA_PATH: &str = "./config/test_mode_data.txt";

    while running.load(Ordering::SeqCst) {
        let test_data_file: File = File::open(TEST_DATA_PATH).unwrap();  
        let reader: BufReader<File> = BufReader::new(test_data_file);

        // Read the file line by line (encoded packet on each line)
        for line in reader.lines() {
            let mut packet_line: String = line.unwrap();  
            packet_line = packet_line[1..packet_line.len()-1].to_string();

            // Parse the packet info into u8 vector 
            let encoded_packet: Vec<u8> = packet_line
                .split(',')
                .map(|num: &str| num.trim().parse::<u8>().unwrap())  
                .collect();

            // if *config::LOGGING.get().unwrap() { println!("{:?}", encoded_packet); }  
            packet_queue.lock().unwrap().push_back(encoded_packet);

            if packet_queue.lock().unwrap().len() >= *config::PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
                api::offload_to_api(packet_queue.clone()); // by reference so offload can empty queue FIFO
            }

            thread::sleep(DELAY);  // Add a delay before the next line
        }
    }
}
fn main() {
    println!("\n{} Loading Config...\n", LOG);
    config::load_config();

    // atomic boolean to track if the program should stop
    let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    // ctrl c handler - so the program will exit the infinite loop
    {
        let r: Arc<AtomicBool> = running.clone();
        ctrlc::set_handler(move || {
            println!("{} Ctrl+C Interrupt Received, shutting down...", LOG);
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    let packet_queue: Arc<Mutex<VecDeque<Vec<u8>>>> = Arc::new(Mutex::new(VecDeque::<Vec<u8>>::new()));
    let queue_clone: Arc<Mutex<VecDeque<Vec<u8>>>> = packet_queue.clone();
    let running_clone_4hb: Arc<AtomicBool> = running.clone();
    let mut system: System = System::new_all();

    thread::spawn(move || {
        heartbeat::heartbeat(running_clone_4hb, queue_clone, &mut system);
    });

    println!("\n{} Starting Sensor (Serial: {})!\n", LOG, config::SERIAL_ID.get().unwrap());
    println!("{}", *config::TEST_MODE.get().unwrap());
    if !*config::TEST_MODE.get().unwrap() {
        // capture packets, parse them, and periodically send to api
        parse_offload(running.clone(), packet_queue);
    } else {
        println!("\nRunning in simulated test mode...");
        test_simulation(running.clone(), packet_queue);
    }   
    println!("\n{} Sensor shut down.\n", LOG);
}
