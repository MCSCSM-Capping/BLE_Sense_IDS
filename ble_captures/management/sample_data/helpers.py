from django.conf import settings
from client.models import *
from avro.io import DatumReader, DatumWriter
from avro.datafile import DataFileReader
import os

PACKETS_DIR = settings.BASE_DIR / "management" / "sample_data" / "packets"

# example of how to open and read the schema
# from avro.schema import parse
# schema = parse(open(PACKETS_DIR / "timeMacPair.avsc", "rb").read().decode())


def load_packets():
    """Makes sample Groups Scanners and packets with some of the cached avro data"""
    for file_name in os.listdir(PACKETS_DIR):
        file_name: str
        if file_name.endswith(".avro"):
            reader = DataFileReader(open(PACKETS_DIR / file_name, "rb"), DatumReader())
            group = Group(name=file_name.removesuffix(".avro"))
