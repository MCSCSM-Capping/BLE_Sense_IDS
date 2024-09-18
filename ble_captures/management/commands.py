from django.core.management.base import BaseCommand, CommandError, CommandParser
from management.load_packets import make_packets
from django.db import transaction
from django.conf import settings
import os


class Command(BaseCommand):
    help = "Adds non-existing class information to the database"

    @transaction.atomic
    def handle(self, *_, **options) -> None:
        BANNER_DUMP_PATH = os.path.join(settings.BASE_DIR, "banner", "data", "classes")

        self.stdout.write(self.style.SUCCESS("Added the packets"))
