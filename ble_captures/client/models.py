from django.db import models
from datetime import date
from django.contrib.auth.models import User

# the naming convention for django classes to that there are singular
#    the is because you will be querying them like so
# ```
# User.objects.all()
# ```

from dataclasses import dataclass
from datetime import date, timedelta



class Company(models.Model):
    __tablename__ = "Companies"
    name = models.TextField()

    registries: models.QuerySet["Registry"]


# grouper for user to register with many companies
#   and companies to have many users
class Registry(models.Model):
    __tablename__ = "Registers"
    password = models.TextField()

    user = models.ForeignKey(User, on_delete=models.CASCADE, related_name="registries")
    company = models.ForeignKey(
        Company, on_delete=models.CASCADE, related_name="registries"
    )


class Group(models.Model):
    __tablename__ = "Groups"
    name = models.TextField()

    scanners: models.QuerySet["Scanner"]


class Scanner(models.Model):
    __tablename__ = "Scanners"
    name = models.TextField()

    group = models.ForeignKey(Group, on_delete=models.CASCADE, related_name="scanners")
    company = models.ForeignKey(
        Company, on_delete=models.CASCADE, related_name="scanners"
    )

    packets: models.QuerySet["Packet"] #One to many


class Device(models.Model):
    __tablename__ = "Devices"


class Packet(models.Model):
    __tablename__ = "Packets"
    advertising_address = models.TextField()
    power_level = models.FloatField()       
    company_id = models.TextField()
    time_stamp = models.DateField()
    rssi = models.IntegerField()
    channel_index = models.IntegerField()
    counter = models.IntegerField()
    protocol_version = models.IntegerField()
    malicious = models.BooleanField()
    long_name = models.TextField()
    oui = models.TextField()
    device = models.ForeignKey(
        Device, on_delete=models.CASCADE, related_name="packets")

    scanner = models.ForeignKey(Scanner, on_delete=models.CASCADE)

class Uuid(models.Model):
    __tablename__ = "UUIDs"
    uuid = models.IntegerField()
    device = models.ForeignKey(Device, on_delete=models.CASCADE)

class User(models.Model):
    __tablename__ = "Users"
    id = models.IntegerField(primary_key=True)
    user_name = models.TextField()
    user_password = models.TextField()

class Scans(models.Model):
    __tablename__ = "Scans"
    scanner = models.ForeignKey(Scanner, on_delete=models.CASCADE)
    packet = models.ForeignKey(Packet, on_delete=models.CASCADE)

class Heartbeat(models.Model):
    __tablename__ = "Heartbeats"
    scanner = models.ForeignKey(Scanner, on_delete=models.CASCADE)
    used_mem = models.FloatField()
    total_mem = models.FloatField()
    used_swap = models.FloatField()
    total_swap = models.FloatField()
    serial_num = models.IntegerField()
    timestamp = models.DateField()
    total_cpu = models.FloatField()
    disk_info = models.ExpressionList() #Is this right?
    queue_length = models.IntegerField()