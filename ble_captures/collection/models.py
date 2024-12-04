from django.db import models
from django.db.models import QuerySet
from datetime import date
from django.contrib.auth.models import User


class SystemInfo(models.Model):
    __tablename__ = "SystemInformation"
    total_memory = models.FloatField()
    used_memory = models.FloatField()
    total_swap = models.FloatField()
    used_swap = models.FloatField()
    total_cpu_usage = models.FloatField()
    disk_info = models.TextField()
    packet_queue_length = models.IntegerField()
    scanner = models.ForeignKey(
        "client.Scanner", on_delete=models.CASCADE
    )  # added this
    timestamp = models.DateTimeField()  # added this
    network_information: QuerySet["NetworkInfo"]


class NetworkInfo(models.Model):
    __tablename__ = "NetworkInformation"
    interface_name = models.TextField()
    total_received = models.IntegerField()
    total_transmitted = models.IntegerField()
    system_info = models.ForeignKey(
        SystemInfo, on_delete=models.CASCADE, related_name="network_information"
    )
