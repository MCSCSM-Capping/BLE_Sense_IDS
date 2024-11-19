from django.contrib import admin
from client.models import Company, Registry, Group, Scanner, Packet, Device, Uuid, User, Scans, Heartbeat

admin.site.register(Company)
admin.site.register(Registry)
admin.site.register(Group)
admin.site.register(Scanner)
admin.site.register(Packet)
admin.site.register(Device)
admin.site.register(Uuid)
admin.site.register(User)
admin.site.register(Scans)
admin.site.register(Heartbeat)
