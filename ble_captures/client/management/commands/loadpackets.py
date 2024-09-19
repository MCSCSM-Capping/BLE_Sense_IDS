from django.core.management.base import BaseCommand
from client.management.sample_data.helpers import *
from django.db import transaction


class Command(BaseCommand):
    help = "Adds non-existing class information to the database"

    @transaction.atomic
    def handle(self, *_, **options) -> None:
        load_packets()
        self.stdout.write(
            self.style.SUCCESS(
                "Successfully loaded the packets, groups, and a test company"
            )
        )
