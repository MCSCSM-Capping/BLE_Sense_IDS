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
from avro.io import BinaryDecoder, DatumReader
from avro.errors import InvalidAvroBinaryEncoding
from avro import schema
from io import BytesIO

PACKET_SCHEMA = """
{
  "type": "record",
  "name": "BLEData",
  "fields": [
    { "name": "timestamp", "type": "long" },
    { "name": "serial_id", "type": "int" },
    {
      "name": "packets",
      "type": {
        "type": "array",
        "items": {
          "type": "record",
          "name": "BLEPacket",
          "fields": [
            {"name": "timestamp", "type": "double", "_comment": "Packet timestamp in seconds"},
            {"name": "rssi", "type": "int", "_comment": "Received signal strength indication"},
            {"name": "channel_index", "type": "int", "_comment": "BLE channel index (0-39)"},
            {"name": "advertising_address", "type": "long", "_comment": "BLE device adv address"},
            {"name": "company_id", "type": "int", "_comment": "Company identifier from advertisement"},
            {"name": "packet_counter", "type": "long", "_comment": "Packet counter from sensor"},
            {"name": "protocol_version", "type": "int", "_comment": "Version of protocol"},
            {"name": "power_level", "type": "int", "_comment": "Power level of the packet"},
            {"name": "oui", "type": "string", "_comment": "Org/Manufacturer from MAC address"},
            {"name": "long_device_name", "type": "string", "_comment": "Device's chosen name"},
            {"name": "short_device_name", "type": "string", "_comment": "Device's shortened name"},
            {"name": "uuids", "type": "string", "_comment": "List of the device's service profiles"}
          ]
        }
      }
    }
  ]
}"""


HEART_BEAT_SCHEMA = """
{
  "type": "record",
  "name": "HeartbeatMessage",
  "fields": [
    {
      "name": "serial",
      "type": "int"
    },
    {
      "name": "timestamp",
      "type": "string"
    },
    {
      "name": "body",
      "type": {
        "type": "record",
        "name": "SystemInfo",
        "fields": [
          {
            "name": "total_memory",
            "type": "float"
          },
          {
            "name": "used_memory",
            "type": "float"
          },
          {
            "name": "total_swap",
            "type": "float"
          },
          {
            "name": "used_swap",
            "type": "float"
          },
          {
            "name": "total_cpu_usage",
            "type": "float"
          },
          {
            "name": "disk_info",
            "type": {
              "type": "array",
              "items": "string"
            }
          },
          {
            "name": "network_info",
            "type": {
              "type": "array",
              "items": {
                "type": "record",
                "name": "NetworkInfo",
                "fields": [
                  {
                    "name": "interface_name",
                    "type": "string"
                  },
                  {
                    "name": "total_received",
                    "type": "long"
                  },
                  {
                    "name": "total_transmitted",
                    "type": "long"
                  }
                ]
              }
            }
          },
          {
            "name": "packet_queue_length",
            "type": "int"
          }
        ]
      }
    }
  ]
}
"""
packet_schema = schema.parse(PACKET_SCHEMA)
packet_reader = DatumReader(packet_schema)
heart_beat_schema = schema.parse(HEART_BEAT_SCHEMA)
heart_beat_reader = DatumReader(heart_beat_schema)


class SendPacketsSocket(WebsocketConsumer):
    def connect(self):
        self.accept()

    def disconnect(self, close_code):  # pyright: ignore
        pass

    def receive(self, bytes_data):  # pyright: ignore
        print(bytes_data)
        decoder = BinaryDecoder(BytesIO(bytes_data))
        try:
            decoded_data = packet_reader.read(decoder)
        except InvalidAvroBinaryEncoding:
            decoded_data = heart_beat_schema.read(decoder)
        print(decoded_data)
