This document describes how the sensor works.

# Sensor Documentation

### Loading Configuration Data
1. The program begins in main(). The first thing we do is load_config().
2. First, we load the contents of the config.ini file into a hashmap. 
3. Then, we use the information stored in it to set global constants. 
4. Next, we load the avro schema from schema.avsc into a Schema object for later.
5. We then load in our OUI lookup table as a hashmap.
6. Finally, we detect what port the dongle is on.
   1. This is accomplished by using the `nrfurtil device list` command.
7. Simply update the settings in the config.ini to change the program's behavior. For example, switching logging to FALSE or PCAPNG to TRUE to get a pcapng copy of the data.

### Running the sniffer
1. First, we create an atomic boolean that tells the program to stop (leave the infinite loop). This way, we can handle interrupts or stop the program when needed but still have our endless reading and parsing loop.
2. We then call the function that does our sniffing work.
3. This function runs the sniffer (by issuing the `nrfutil ble-sniffer sniff` command). The code will autodetect the OS it is on and adjust accordingly. We force it to put packets to stdout and throw away the pcapng data so that it does not accumulate on the host.
4. Once the sniffer is running, we see if there is stdout for us to capture. If there is, we see if the log statement contains a parsed packet. If it does, we parse the log string into a BLEPacket struct to hold the data we want as an object. We then encode this object using our avro schema and add it to our packet queue. When the packet queue reaches a specified size, we offload the first specified number of packets off to the API (to the backend).
   
### Parsing Packets
1. First, we use regular expressions to extract specific patterns and capture groups from the log string we got from nrfutil. For most of the data, this is simple.
2. Advertising data takes an extra step. We use regex to get the hex data (given to us in decimal for some reason). Then, we iterate though it and attempt to extract data from it based on hex indicators. (For example, if we see a 0xFF in a certain position, company ID follows).

