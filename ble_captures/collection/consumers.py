from django.http import HttpRequest, HttpResponse
from django.shortcuts import redirect, render
from client.models import *
from django.views import View
from dataclasses import dataclass
from django.http import JsonResponse
from django.core.validators import EmailValidator
from django.contrib.auth.password_validation import validate_password
from django.contrib.auth import authenticate, login, logout
from django.core.exceptions import ValidationError
from django.contrib import messages
from django.contrib.auth.models import User
from channels.generic.websocket import WebsocketConsumer
import io
import fastavro
import json
from typing import List
from dataclasses import dataclass

@dataclass
class BLEPacket:
    timestamp: float
    rssi: int
    channel_index: int
    advertising_address: int
    company_id: int
    packet_counter: int
    protocol_version: int
    power_level: int
    oui: str
    long_device_name: str
    short_device_name: str
    uuids: str

@dataclass
class PacketDelivery:
    serial_id: int
    timestamp: int
    packets: List[BLEPacket]

@dataclass
class NetworkInfo:
    interface_name: str
    total_received: int
    total_transmitted: int

@dataclass
class SystemInfo:
    total_memory: float            # Total memory in GB
    used_memory: float             # Used memory in GB
    total_swap: float              # Total swap memory in GB
    used_swap: float               # Used swap memory in GB
    total_cpu_usage: float         # Total CPU usage as percentage
    disk_info: List[str]           # List of disk info strings
    network_info: List[NetworkInfo] # List of network information
    packet_queue_length: int       # Length of packet queue

@dataclass
class HeartbeatMessage:
    serial: int
    timestamp: str
    body: SystemInfo

with open('./collection/packet_schema.avsc', 'r') as packet_schema_file:
    PACKET_SCHEMA = json.load(packet_schema_file)

with open('./collection/hb_schema.avsc', 'r') as hb_schema_file:
    HB_SCHEMA = json.load(hb_schema_file)

def decode_packet_delivery(binary_data: bytes) -> PacketDelivery:
    bytes_reader = io.BytesIO(binary_data)
    reader = fastavro.reader(bytes_reader, PACKET_SCHEMA)
    data = next(reader)
    
    # data -> dataclass struct
    packets = [
        BLEPacket(
            timestamp=p['timestamp'],
            rssi=p['rssi'],
            channel_index=p['channel_index'],
            advertising_address=p['advertising_address'],
            company_id=p['company_id'],
            packet_counter=p['packet_counter'],
            protocol_version=p['protocol_version'],
            power_level=p['power_level'],
            oui=p['oui'],
            long_device_name=p['long_device_name'],
            short_device_name=p['short_device_name'],
            uuids=p['uuids']
        )
        for p in data['packets']
    ]
    
    return PacketDelivery(
        serial_id=data['serial_id'],
        timestamp=data['timestamp'],
        packets=packets
    )
    
def decode_heartbeat(binary_data: bytes) -> HeartbeatMessage:
    bytes_reader = io.BytesIO(binary_data)
    reader = fastavro.reader(bytes_reader, HB_SCHEMA)
    data = next(reader)

    # Map data to the NetworkInfo, SystemInfo, and HeartbeatMessage dataclasses
    network_info = [
        NetworkInfo(
            interface_name=n['interface_name'],
            total_received=n['total_received'],
            total_transmitted=n['total_transmitted']
        )
        for n in data['body']['network_info']
    ]

    system_info = SystemInfo(
        total_memory=data['body']['total_memory'],
        used_memory=data['body']['used_memory'],
        total_swap=data['body']['total_swap'],
        used_swap=data['body']['used_swap'],
        total_cpu_usage=data['body']['total_cpu_usage'],
        disk_info=data['body']['disk_info'],
        network_info=network_info,
        packet_queue_length=data['body']['packet_queue_length']
    )

    return HeartbeatMessage(
        serial=data['serial'],
        timestamp=data['timestamp'],
        body=system_info
    )

class SendPacketsSocket(WebsocketConsumer):
    def connect(self):
        self.accept()

    def disconnect(self, close_code):  # pyright: ignore
        pass

    def receive(self, bytes_data):  # pyright: ignore
        # print(bytes_data)
        print("Received Transmission")
        try:
            packet_delivery = decode_packet_delivery(bytes_data)
            print("First deserialized packet from delivery: ")
            print(packet_delivery.packets[0])
        except:
            hb_msg = decode_heartbeat(bytes_data)
            print(hb_msg)

