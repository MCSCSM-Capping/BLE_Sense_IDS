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

with open('./collection/packet_schema.avsc', 'r') as packet_schema_file:
    PACKET_SCHEMA = json.load(packet_schema_file)

def decode_avro_packet(binary_data: bytes) -> PacketDelivery:
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
    
class SendPacketsSocket(WebsocketConsumer):
    def connect(self):
        self.accept()

    def disconnect(self, close_code):  # pyright: ignore
        pass

    def receive(self, bytes_data):  # pyright: ignore
        # print(bytes_data)
        print("Received Transmission")
        try:
            packet_delivery = decode_avro_packet(bytes_data)
            print("First deserialized packet from delivery: ")
            print(packet_delivery.packets[0])
        except:
            print("Heartbeat")

