from collection.models import *
from client.models import *
from channels.generic.websocket import WebsocketConsumer
import io
import os
from django.conf import settings
import fastavro
import json
from typing import TypedDict
import logging


class BLEPacketDict(TypedDict):
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


class PacketDeliveryDict(TypedDict):
    serial_id: int
    timestamp: int
    packets: list[BLEPacketDict]


class NetworkInfoDict(TypedDict):
    interface_name: str
    total_received: int
    total_transmitted: int


class SystemInfoDict(TypedDict):
    total_memory: float  # Total memory in GB
    used_memory: float  # Used memory in GB
    total_swap: float  # Total swap memory in GB
    used_swap: float  # Used swap memory in GB
    total_cpu_usage: float  # Total CPU usage as percentage
    disk_info: str  # disk info strings separated by commas
    packet_queue_length: int  # Length of packet queue


class HeartbeatMessageDict(TypedDict):
    serial: int
    timestamp: str
    body: SystemInfoDict
    network_info: list[NetworkInfoDict]  # List of network information


with open(
    os.path.join(settings.BASE_DIR, "collection", "packet_schema.avsc"), "r"
) as packet_schema_file:
    PACKET_SCHEMA = json.load(packet_schema_file)

with open(
    os.path.join(settings.BASE_DIR, "collection", "hb_schema.avsc"), "r"
) as hb_schema_file:
    HB_SCHEMA = json.load(hb_schema_file)


def decode_packet_delivery(binary_data: bytes) -> PacketDeliveryDict:
    bytes_reader = io.BytesIO(binary_data)
    reader = fastavro.reader(bytes_reader, PACKET_SCHEMA)
    data: dict = next(reader)  # pyright: ignore

    # data -> dataclass struct
    packets: list[BLEPacketDict] = [
        {
            "timestamp": p["timestamp"],
            "rssi": p["rssi"],
            "channel_index": p["channel_index"],
            "advertising_address": p["advertising_address"],
            "company_id": p["company_id"],
            "packet_counter": p["packet_counter"],
            "protocol_version": p["protocol_version"],
            "power_level": p["power_level"],
            "oui": p["oui"],
            "long_device_name": p["long_device_name"],
            "short_device_name": p["short_device_name"],
            "uuids": p["uuids"],
        }
        for p in data["packets"]
    ]

    return {
        "serial_id": data["serial_id"],
        "timestamp": data["timestamp"],
        "packets": packets,
    }


def decode_heartbeat(binary_data: bytes) -> HeartbeatMessageDict:
    bytes_reader = io.BytesIO(binary_data)
    reader = fastavro.reader(bytes_reader, HB_SCHEMA)
    data: dict = next(reader)  # pyright: ignore

    # Map data to the NetworkInfo, SystemInfo, and HeartbeatMessage dataclasses
    network_info: list[NetworkInfoDict] = [
        {
            "interface_name": n["interface_name"],
            "total_received": n["total_received"],
            "total_transmitted": n["total_transmitted"],
        }
        for n in data["body"]["network_info"]
    ]

    system_info: SystemInfoDict = {
        "total_memory": data["body"]["total_memory"],
        "used_memory": data["body"]["used_memory"],
        "total_swap": data["body"]["total_swap"],
        "used_swap": data["body"]["used_swap"],
        "total_cpu_usage": data["body"]["total_cpu_usage"],
        "disk_info": ", ".join(data["body"]["disk_info"]),
        "packet_queue_length": data["body"]["packet_queue_length"],
    }

    return {
        "serial": data["serial"],
        "timestamp": data["timestamp"],
        "body": system_info,
        "network_info": network_info,
    }


class SendPacketsSocket(WebsocketConsumer):
    def connect(self):
        self.accept()

    def disconnect(self, close_code):  # pyright: ignore
        pass

    def receive(self, bytes_data):  # pyright: ignore
        # print(bytes_data)
        logging.info("Received Transmission")
        # there might be a better way to decied which one is which
        try:
            packet_delivery = decode_packet_delivery(bytes_data)
            logging.info("First deserialized packet from delivery: ")
            logging.info(packet_delivery["packets"][0])
        except:
            hb_msg = decode_heartbeat(bytes_data)
            system_info = SystemInfo(**hb_msg["body"])
            system_info.save()
            network_information = hb_msg["network_info"]
            network_objs = [
                NetworkInfo(**n, system_info=system_info) for n in network_information
            ]
            NetworkInfo.objects.bulk_create(network_objs)
            logging.info("Recieved HB message")
            logging.info("")