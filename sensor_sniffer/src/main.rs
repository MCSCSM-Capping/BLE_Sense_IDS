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
    sync::Arc,
    time::Duration,
};
use tokio::{sync::Mutex, signal};
use config::BLEPacket;
use tester::generate_random_packet;
use log::{info, trace, error};
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
async fn parse_offload(packet_queue: Arc<Mutex<VecDeque<BLEPacket>>>) {
    // atomic boolean to track if process should stop
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
                                        
                    trace!("\n\n{}{}{:#?}\n", LOG, line, parsed_ble_packet);
                    // Lock once for both push_back and checking the size
                    let mut locked_queue = packet_queue.lock().await;

                    locked_queue.push_back(parsed_ble_packet);
                    // trace!("Queue Size: {}", queue.len());

                    if locked_queue.len() >= *config::PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
                        tokio::spawn(socket::deliver_packets(packet_queue.clone())); // we don't care when this completes
                    }

                    drop(locked_queue); // drop the queue lock
                }
            }
        }
    }
    info!("Packet Process Halted.");
    // dump_queue(); if shutting down, might want to dump the queue to api first to avoid data loss?
}


async fn test_simulation(packet_queue: Arc<Mutex<VecDeque<BLEPacket>>>) {
    const DELAY: Duration = Duration::from_millis(10);

    // Use a loop to continuously simulate until we get a Ctrl+C signal
    loop {
        // Check for Ctrl+C signal
        tokio::select! {
            // Simulate packet generation
            _ = tokio::time::sleep(DELAY) => {
                let simulated_packet: BLEPacket = generate_random_packet();

                trace!("\n\n{}{:#?}\n", LOG, simulated_packet);

                // Lock the queue and push the new simulated packet
                let mut locked_queue = packet_queue.lock().await;
                locked_queue.push_back(simulated_packet);

                // If the queue exceeds the specified buffer size, spawn a task to deliver packets
                if locked_queue.len() >= *config::PACKET_BUFFER_SIZE.get().expect("PACKET_BUFFER_SIZE is not initialized") as usize {
                    tokio::spawn(socket::deliver_packets(packet_queue.clone()));
                }

                // Explicitly drop the lock (this is optional, as it will drop automatically when the scope ends)
                drop(locked_queue);
            },

            // Handle Ctrl+C signal (gracefully terminate the simulation)
            _ = signal::ctrl_c() => {
                info!("Ctrl+C received by simulation.");
                break; // Exit the loop when the interrupt signal is received
            }
        }
    }

    info!("Test Simulation Process Halted.");
}

#[tokio::main]
async fn main() {
    // initialize logger
    env_logger::init();

    info!("{} Loading Sensor Configuration...", LOG);
    config::load_config();

    info!("{} Starting Sensor (Serial: {})!", LOG, config::SERIAL_ID.get().unwrap());

    let packet_queue: Arc<Mutex<VecDeque<BLEPacket>>> = Arc::new(Mutex::new(VecDeque::<BLEPacket>::new()));
    let queue_clone: Arc<Mutex<VecDeque<BLEPacket>>> = packet_queue.clone();

    // start heartbeat
    let hb_handle = tokio::spawn(heartbeat::heartbeat(queue_clone));

    info!("Successfully Spawned Heartbeat Thread.");

    // Start the packet parsing or test simulation task based on config
    let main_task_handle = if !*config::TEST_MODE.get().unwrap() {
        // Capture packets, parse them, and periodically send to socket
        info!("NRF Packet Processing Started...");
        tokio::spawn(parse_offload(packet_queue))
    } else {
        // Running in test mode
        info!("Packet Simulation Started...");
        tokio::spawn(test_simulation(packet_queue))
    };

    // Await main task and hb concurrently
    let hb_result = hb_handle.await;
    let main_task_result = main_task_handle.await;

    // errors from the tasks
    if let Err(e) = hb_result {
        error!("Error in heartbeat task: {:?}", e);
    }
    if let Err(e) = main_task_result {
        error!("Error in task (parse_offload or test_simulation): {:?}", e);
    }

    info!("{} All tasks halted safely. Sensor shut down.\n", LOG);
}
    