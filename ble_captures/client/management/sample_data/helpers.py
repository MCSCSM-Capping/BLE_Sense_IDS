from django.conf import settings
from client.models import *
from avro.io import DatumReader, DatumWriter
from avro.datafile import DataFileReader
from datetime import datetime
import os
import random

PACKETS_DIR = settings.BASE_DIR / "client" / "management" / "sample_data" / "packets"

# example of how to open and read the schema
# from avro.schema import parse
# schema = parse(open(PACKETS_DIR / "timeMacPair.avsc", "rb").read().decode())


def load_packets():
    """Makes sample Groups Scanners and packets with some of the cached avro data"""
    company = Company(name="Test Company")
    company.save()
    for file_name in os.listdir(PACKETS_DIR):
        file_name: str
        if file_name.endswith(".avro"):
            name = file_name.removesuffix(".avro")
            group = Group(name=name)
            group.save()

            scanners: list[Scanner] = []
            for i in range(random.randint(1, 5)):
                scanner = Scanner(name=name + str(i), group=group, company=company)
                scanner.save()
                scanners.append(scanner)
            reader = DataFileReader(open(PACKETS_DIR / file_name, "rb"), DatumReader())
            packets = []
            MAX_PACKETS = 100_000
            i = 0
            for packet_obj in reader:
                # currently only one packet per and the scanner is a random one
                packet = Packet(
                    time_stamp=datetime.fromtimestamp(packet_obj.get("time_stamp")),  # pyright: ignore
                    advertising_address=packet_obj.get("advertising_address"),  # pyright: ignore
                    power_level=packet_obj.get("power_level"),  # pyright: ignore
                    rssi=packet_obj.get("power_level"),  # pyright: ignore
                    channel_index=random.randrange(0, 20),
                    counter=random.randrange(0, 100),
                    protocol_version=random.randrange(0, 7),
                    malicious=random.randint(0, 1) == 0,
                    company_id=packet_obj.get("company_id", 0),  # pyright: ignore
                    scanner=scanners[random.randrange(0, len(scanners))],
                )
                packets.append(packet)
                i += 1
                if i > MAX_PACKETS:
                    break
            Packet.objects.bulk_create(packets, batch_size=10_000)
