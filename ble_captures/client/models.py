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

    registries: models.QuerySet["Registy"]


# grouper for user to register with many companies
#   and companies to have many users
class Registy(models.Model):
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

    packets: models.QuerySet["Packet"]


class Packet(models.Model):
    __tablename__ = "Packets"
    mac_address = models.TextField()
    mac_frequencey = models.FloatField()
    company = models.TextField()
    # TODO: dicuss this
    #   should this be time since the recording started?
    #           -- how would we even get that?
    #   should this be time since Epoch?
    timestamp = models.IntegerField()

    scanner = models.ForeignKey(Scanner, on_delete=models.CASCADE)
