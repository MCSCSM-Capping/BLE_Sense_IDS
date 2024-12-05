from collection.models import *
from client.models import *
from channels.generic.websocket import WebsocketConsumer
from io import BytesIO
from fastavro._read_common import SchemaResolutionError
from collections import defaultdict
import os
from datetime import datetime
from django.conf import settings
import fastavro
import json
from typing import TypedDict
from collection.ml_model.bleClassification import classify
import logging

# some important constants for the algorithm
BUFFER_SIZE_IN_SECONDS = 0.5
# MIN_BUFFER_COUNT = 20

# cannot be longer than this
MIN_TIME_TO_BE_RELATED = 0.1


class BLEPacketDict(TypedDict):
    advertising_address: str
    power_level: int
    company_id: int
    time_stamp: datetime
    rssi: int
    channel_index: int
    counter: int
    protocol_version: str
    long_name: str
    short_name: str
    oui: str
    uuids: str
    scanner: Scanner


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
    timestamp: datetime
    packet_queue_length: int  # Length of packet queue
    scanner: Scanner


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


def get_or_create_scanner(scanner_id) -> Scanner:
    scanner = Scanner.objects.filter(id=scanner_id).first()
    if scanner is not None:
        return scanner

    group = Group.objects.create(name="Lowell Thmoas")
    company = Company.objects.create(name="Marist")

    return Scanner.objects.create(group=group, company=company)


def decode_packet_delivery(bytes_reader: BytesIO) -> PacketDeliveryDict:
    reader = fastavro.reader(bytes_reader, PACKET_SCHEMA)
    data: dict = next(reader)  # pyright: ignore

    scanner = get_or_create_scanner(data["serial_id"])
    # data -> dataclass struct
    logging.info(data["packets"][0]["timestamp"])
    packets: list[BLEPacketDict] = [
        {
            "advertising_address": p["advertising_address"],
            "power_level": p["power_level"],
            "company_id": p["company_id"],
            "time_stamp": datetime.fromtimestamp(p["timestamp"]),
            "rssi": p["rssi"],
            "channel_index": p["channel_index"],
            "counter": p["packet_counter"],
            "protocol_version": p["protocol_version"],
            "long_name": p["long_device_name"],
            "short_name": p["short_device_name"],
            "oui": p["oui"],
            "uuids": p["uuids"],
            "scanner": scanner,
        }
        for p in data["packets"]
    ]

    return {
        "serial_id": data["serial_id"],
        "timestamp": data["timestamp"],
        "packets": packets,
    }


def decode_heartbeat(bytes_reader: BytesIO) -> HeartbeatMessageDict:
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
        "scanner": get_or_create_scanner(data["serial"]),
        "timestamp": datetime.fromtimestamp(int(data["timestamp"])),
    }

    return {
        "serial": data["serial"],
        "timestamp": data["timestamp"],
        "body": system_info,
        "network_info": network_info,
    }


@dataclass
class BufferPacket:
    packet_pk: int
    timestamp: float
    advertising_address: str


@dataclass
class DeviceListing:
    device_pk: int
    last_timestamp: float
    lastest_address: str


@dataclass
class AddressHueristic:
    count: int
    lastest_timestamp: float


class PacketAnalysisBuffer:
    def __init__(self) -> None:
        # could look at using some better datastructures
        self.packets_in_buffer: list[BufferPacket] = []
        self.sorted_devices: list[DeviceListing] = []
        self.hueristic_in_buffer: dict[str, AddressHueristic] = {}
        self.last_packet_acceptance: float = datetime.now().timestamp()

    def accept_packets(self, delivery: PacketDeliveryDict):
        self.last_packet_acceptance = datetime.now().timestamp()
        logging.info("Accpeting packets")
        classifications = classify(delivery["packets"])
        logging.info("Finished with the ML model")
        packets = []
        for is_malcious, packet in zip(classifications, delivery["packets"]):
            packets.append(Packet(malicious=is_malcious, **packet))
        insert_packets = Packet.objects.bulk_create(packets)
        logging.info(f"added {len(insert_packets)} to the database")
        buffer_packets: list[BufferPacket] = []

        for p in insert_packets:
            # do not place adveisting address with the default value
            if p.advertising_address == "-1" or p.advertising_address == -1:
                continue

            hueristic = self.hueristic_in_buffer.get(p.advertising_address)
            if hueristic is None:
                hueristic = AddressHueristic(
                    count=0, lastest_timestamp=p.time_stamp.timestamp()
                )
                self.hueristic_in_buffer[p.advertising_address] = hueristic
            else:
                hueristic.lastest_timestamp = p.time_stamp.timestamp()

            hueristic.count += 1
            buffer_packets.append(
                BufferPacket(
                    packet_pk=p.pk,
                    advertising_address=p.advertising_address,
                    timestamp=p.time_stamp.timestamp(),
                )
            )

        self.packets_in_buffer.extend(buffer_packets)
        self.update()

    # could call this function more often
    # this function will release packets from the buffer if need as well as
    #    release devices that are no longer needed
    def update(self):
        logging.info("Updating packet")
        edge_time = self.last_packet_acceptance - BUFFER_SIZE_IN_SECONDS
        # a different datastructure than a python list probably makes more since
        #    this operation is O(n) to remove from the front
        og_length = len(self.packets_in_buffer)
        while (
            len(self.packets_in_buffer) > 0
            and self.packets_in_buffer[0].timestamp < edge_time
        ):
            self.place_packet(self.packets_in_buffer.pop(0))
        logging.info(f"Placed {og_length - len(self.packets_in_buffer)} packets")

    def place_packet(self, packet: BufferPacket):
        self.hueristic_in_buffer[packet.advertising_address].count -= 1
        device = self.get_remove_device(packet)
        device.last_timestamp = self.last_packet_acceptance
        device.lastest_address = packet.advertising_address
        self.sorted_devices.append(device)
        packet_db = Packet.objects.get(id=packet.packet_pk)
        packet_db.device = Device.objects.get(id=device.device_pk)
        packet_db.save()

    # handles getting the packet listing and removing it from the current list
    #  also will add one to the database if needed
    def get_remove_device(self, packet: BufferPacket) -> DeviceListing:
        # always link to the same mac address
        index = self.get_device_index(packet.advertising_address)
        if index is not None:
            return self.sorted_devices.pop(index)
        current_time = self.last_packet_acceptance
        # look for the best link
        i = -1
        while (len(self.sorted_devices) >= abs(i)) and (
            self.sorted_devices[i].last_timestamp
            > current_time - MIN_TIME_TO_BE_RELATED
        ):
            # if there is more packets in the buffer with the canidate link then that means
            #   it is not a link because the device did not change mac address
            if packet.advertising_address not in self.hueristic_in_buffer or not (
                self.hueristic_in_buffer[packet.advertising_address].count > 0
            ):
                # create a link
                return self.sorted_devices.pop(i)
            i -= 1
        # create new device as there is no link
        device_db = Device.objects.create()
        device = DeviceListing(
            device_pk=device_db.pk,
            last_timestamp=current_time,
            lastest_address=packet.advertising_address,
        )
        return device

    # could implement a fastest search if you can sort by both
    def get_device_index(self, address) -> int | None:
        for i, device_listing in enumerate(self.sorted_devices):
            if device_listing.lastest_address == address:
                return i
        return None


class SendPacketsSocket(WebsocketConsumer):
    def connect(self):
        self.accept()
        self.analysis: PacketAnalysisBuffer = PacketAnalysisBuffer()

    def disconnect(self, close_code):  # pyright: ignore
        pass

    def receive(self, bytes_data):  # pyright: ignore
        # print(bytes_data)
        logging.info("Received Transmission")
        try:
            logging.info("Recieved Packet Message")
            packet_delivery = decode_packet_delivery(BytesIO(bytes_data))
            logging.info(
                f"First deserialized packet from delivery: {packet_delivery["packets"][0]}"
            )
            self.analysis.accept_packets(packet_delivery)
        except (ValueError, SchemaResolutionError) as e:
            # logging.info(f"Failed loading as packets {e}")
            logging.info("Recieved HB message")
            try:
                hb_msg = decode_heartbeat(BytesIO(bytes_data))
            except (ValueError, SchemaResolutionError) as e:
                logging.error(f"Failed to validate either schema HR error: {e}")
                return
            system_info = SystemInfo(**hb_msg["body"])
            system_info.save()
            network_information = hb_msg["network_info"]
            network_objs = [
                NetworkInfo(**n, system_info=system_info) for n in network_information
            ]
            NetworkInfo.objects.bulk_create(network_objs)
