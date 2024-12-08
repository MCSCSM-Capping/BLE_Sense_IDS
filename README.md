# BLE_Sense
This project is a Marist College Capping project for Fall 2024. The goal is to develop an intrusion detection system (IDS) for Bluetooth Low Energy (BLE) devices. 

# Project Components
1. A sensor (written in Rust) that runs on a small form factor & power efficient device (Zima board). This sensor captures raw BLE packet data using a Nordic RF52840 Dongle and the nrfutil software. These raw packets are processed, cleaned, and reduced to a relevant set of attributes. These packets along with heartbeat messages (CPU, RAM, etc) are encoded (using Apache Avro) and delivered to the backend via a websocket.
2. Our backend receives this data, decodes it, and stores in in our database. We then run a device algorithm on this data to link packets from the same devices together (BLE packets utilize MAC address randomization so an algorithm to crack this pattern is necessary). Hosted on the backend is a machine learning model developed by a different capping team to identify packets as benign or malicious. 
3. Our front end displays this data in a human readable format to our users. It reports on devices on the network, malicious packets/devices, attacks, and more to form a powerful, effective, and first of its kind BLE IDS.

# Documentation & Demo
Relevant documentation for each project component including setup information is available in the Documentation folder. A high-level overview is available below.
A demo of our project can be found [here](https://youtu.be/kEsnQSQ_-JM).

![Basic Overview](./Documentation/High%20Level%20Arch-Dataflow.png)
