This document describes how the sensor works.

# Sensor Documentation
![Sensor Dataflow](./Sensor%20Dataflow.png)

### Loading Configuration Data
1. The program begins in main(). The first thing we do is load_config().
2. First, we load the contents of the config.ini file into a hashmap. 
3. Then, we use the information stored in it to set global constants. 
4. Next, we load the avro schema for both Packet Deliveries and Heartbeat Messages into schema objects.
5. We then load in our OUI lookup table as a hashmap.
6. We also open the websocket connection to the backend to use later.
7. Finally, we detect what port the dongle is on.
   1. This is accomplished by using the `nrfurtil device list` command.
8. Simply update the settings in the config.ini to change the program's behavior. For example, switching OFFLINE to TRUE or PCAPNG to TRUE to get a pcapng copy of the data.

### Running the sniffer
1. First, we initialize the logger and load the config.
2. Next, we start the heartbeat process.
3. We then async call the function that does our sniffing work (either the simulated test mode or real capture).
   1. Test Mode: Simply generates random packets (using rand, the data should largely still be valid though).
   2. At a set interval it simulates another packet and adds it to the queue.
   3. All other behavior is the same!
4. The main sniffing function runs the sniffer (by issuing the `nrfutil ble-sniffer sniff` command). The code will autodetect the OS it is on and adjust accordingly. We force nrfutil to put packets to stdout and throw away the pcapng data so that it does not accumulate on the host.
5. Once the sniffer is running, we see if there is stdout for us to capture. If there is, we see if the log statement contains a parsed packet. If it does, we parse the log string into a BLEPacket struct to hold the data we want as an object. We then add this parsed packet to our packet queue. When the packet queue reaches a specified size, we offload the first specified number of packets off to the backend via the websocket connection (as an object that contains some header information - we encode it using avro schema).
   
### Parsing Packets
1. First, we use regular expressions to extract specific patterns and capture groups from the log string we got from nrfutil. For most of the data, this is simple.
2. Advertising data takes an extra step. We use regex to get the hex data (given to us in decimal for some reason). Then, we iterate though it and attempt to extract data from it based on hex indicators. (For example, if we see a 0xFF in a certain position, company ID follows).

### Heartbeat 
1. The heartbeat messages are managed independently by an async process that never rejoins main until an interrupt is received. 
2. The message reports to the backend how the sensor is doing (CPU, RAM, Queue length, etc.)
3. It should work on both linux and windows. The queue is borrowed using a mutex to avoid thread issues.
