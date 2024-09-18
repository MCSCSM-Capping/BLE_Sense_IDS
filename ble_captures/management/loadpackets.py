from django.core.management.base import BaseCommand, CommandError, CommandParser
from management.sample_data.helpers import *
from django.db import transaction
from django.conf import settings
import os


class Command(BaseCommand):
    help = "Adds non-existing class information to the database"

    @transaction.atomic
    def handle(self, *_, **options) -> None:
        PACKET_DIR = os.path.join(settings.BASE_DIR, "management")
