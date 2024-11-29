mod heartbeat;
mod packet_parser;
mod config;
mod socket;
mod tester;
use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use config::BLEPacket;
use sysinfo::System;
use tester::generate_random_packet;
use log::{info, trace};
use env_logger;
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
    info!("{} nrfSniffer started with PID: {}", LOG, sniffer.id());
    return sniffer; // return process so we can reference it later
}

// constantly take in the output of nrfutil. stop with an interrupt. manage api sending.
fn parse_offload(running: Arc<AtomicBool>, packet_queue: Arc<Mutex<VecDeque<BLEPacket>>>) {
    let mut sniffer: Child = start_nrf_sniffer(); // start sniffer

    // we do this loop forever (or until interrupt)
    while running.load(Ordering::SeqCst) {
        // capture the nrf info from stdout
        if let Some(stdout) = sniffer.stdout.take() {
            let reader = BufReader::new(stdout);
            // Read the stdout line by line as it comes in
            for line in reader.lines() {
                let line: String = line.expect("Could not read line from stdout");

                if line.contains("Parsed packet") {
                    let parsed_ble_packet: config::BLEPacket = packet_parser::parse_ble_packet(&line); 
                                        
                    trace!("\n\n{}{}{:#?}", LOG, line, parsed_ble_packet);

                    packet_queue.lock().unwrap().push_back(parsed_ble_packet);
                    // trace!("Queue Size: {}", queue.len());
                }

                if packet_queue.lock().unwrap().len() >= *config::PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
                    socket::deliver_packets(packet_queue.clone()); // by reference so offload can empty queue FIFO
                }
            }
        }
    }
    // dump_queue(); if shutting down, might want to dump the queue to api first to avoid data loss?
}

fn test_simulation(running: Arc<AtomicBool>, packet_queue: Arc<Mutex<VecDeque<BLEPacket>>>) {
    const DELAY: Duration = Duration::from_millis(10);

    while running.load(Ordering::SeqCst) {
        let simulated_packet: BLEPacket = generate_random_packet();

        trace!("\n\n{}{:#?}", LOG, simulated_packet);
        
        packet_queue.lock().unwrap().push_back(simulated_packet);

        if packet_queue.lock().unwrap().len() >= *config::PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
            socket::deliver_packets(packet_queue.clone()); // by reference so offload can empty queue FIFO
        }

        thread::sleep(DELAY);  // Add a delay before adding more
    }
}

fn main() {
    // initialize logger
    env_logger::init();

    info!("\n{} Loading Sensor Configuration...\n", LOG);
    config::load_config();

    // atomic boolean to track if the program should stop
    let running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
    // ctrl c handler - so the program will exit the infinite loop
    // simply sets atomic bool to false so that we exit safely
    {
        let r: Arc<AtomicBool> = running.clone();
        ctrlc::set_handler(move || {
            info!("{} Ctrl+C Interrupt Received, shutting down...", LOG);
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    info!("\n{} Starting Sensor (Serial: {})!\n", LOG, config::SERIAL_ID.get().unwrap());

    let packet_queue: Arc<Mutex<VecDeque<BLEPacket>>> = Arc::new(Mutex::new(VecDeque::<BLEPacket>::new()));
    let queue_clone: Arc<Mutex<VecDeque<BLEPacket>>> = packet_queue.clone();
    let running_clone_4hb: Arc<AtomicBool> = running.clone();
    // system obj used to collect load & resource information
    let mut system: System = System::new_all();

    thread::spawn(move || { // start the heart beating
        heartbeat::heartbeat(running_clone_4hb, queue_clone, &mut system);
    });
    info!("Successfully Spawned Hearbeat Thread.");

    if !*config::TEST_MODE.get().unwrap() {
        // capture packets, parse them, and periodically send to socket
        parse_offload(running.clone(), packet_queue);
    } else {
        info!("\nRunning in simulated test mode...");
        test_simulation(running.clone(), packet_queue);
    }   
    info!("\n{} Sensor shut down.\n", LOG);
}
    