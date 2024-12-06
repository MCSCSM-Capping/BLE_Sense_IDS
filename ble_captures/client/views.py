from django.http import HttpRequest, HttpResponse
from django.shortcuts import redirect, render
from client.models import *
from collection.models import *
from django.views import View
from dataclasses import dataclass, asdict
from django.http import JsonResponse
from django.core.validators import EmailValidator
from django.contrib.auth.password_validation import validate_password
from django.contrib.auth import authenticate, login, logout
from django.core.exceptions import ValidationError
from django.contrib import messages
from django.contrib.auth.models import User
from django.core.serializers import serialize
from django.db.models import Max
import json
from django.db.models import Count, Q, Exists, OuterRef
from datetime import datetime, timedelta
from django.utils import timezone
from django.utils.timezone import now
from django.core.paginator import Paginator
from django.db.models import Case, When, F, FloatField


# queries from DB------------------------------------------------------------------------------------------------------------------------------------------------------------------


def sys_status(request):
    # Fetch each scanner's ID, name, and the latest timestamp of system info
    latest_system_info = Scanner.objects.annotate(
        latest_timestamp=Max("systeminfo__timestamp")
    ).values("id", "name", "latest_timestamp")

    # Return as JSON response
    return JsonResponse(list(latest_system_info), safe=False)


def system_metrics(request, scanner_id):
    time_threshold = now() - timedelta(seconds=90)

    # Fetch the latest heartbeat entry within the last 90 seconds for the specified scanner
    heartbeat = (
        SystemInfo.objects.filter(scanner_id=scanner_id, timestamp__gte=time_threshold)
        .order_by("-timestamp")
        .first()
    )

    # If no heartbeat data exists within the last 90 seconds for the given scanner ID return error with the last heartbeat time ever
    if not heartbeat:
        last_heartbeat = (
            SystemInfo.objects.filter(scanner_id=scanner_id)
            .order_by("-timestamp")
            .first()
        )

        hbtime = last_heartbeat.timestamp if last_heartbeat else 0
        data = {
            "error": "No heartbeat data found for this scanner in the last 90 seconds",
            "time": hbtime,
        }
        return JsonResponse(data)

    mem_perc = round((heartbeat.used_memory / heartbeat.total_memory) * 100, 2)
    swap_perc = round((heartbeat.used_swap / heartbeat.total_swap) * 100, 2)
    total_cpu = round(heartbeat.total_cpu_usage, 2)

    # Prepare the response data with the relevant metrics
    data = {
        "mem_perc": mem_perc,
        "swap_perc": swap_perc,
        "total_cpu": total_cpu,
    }

    # Return the data in a JSON response
    return JsonResponse(data)


# for line graph
def device_count(request):
    # Get the current time and the time 60 seconds ago
    now = timezone.now()
    sixty_seconds_ago = now - timedelta(seconds=60)

    # Filter packets from the last 60 seconds
    recent_packets_filter = Q(packets__time_stamp__gte=sixty_seconds_ago)

    # Annotate devices with packet counts
    annotated_devices = Device.objects.filter(recent_packets_filter).annotate(
        total_packets=Count("packets", filter=recent_packets_filter),
        malicious_packets=Count("packets", filter=recent_packets_filter & Q(packets__malicious=True)),
        malicious_percentage=Case(
            When(total_packets__gt=0, then=(F("malicious_packets") * 100.0) / F("total_packets")),
            default=0.0,
            output_field=FloatField(),
        ),
    )

    # Count all devices
    all_device_count = annotated_devices.count()

    # Count malicious devices where 60% or more packets are malicious
    # change threshold value here
    malicious_device_count = annotated_devices.filter(malicious_percentage__gte=60).count()

    # Calculate non-malicious devices as total minus malicious
    non_malicious_device_count = all_device_count - malicious_device_count

    # Response with counts
    return JsonResponse(
        {
            "all_devices": all_device_count,
            "non_malicious_devices": non_malicious_device_count,
            "malicious_devices": malicious_device_count,
        }
    )



# for donut chart and table
def device_stats(request):
    # Parse the start and end dates from the query parameters
    start_date = request.GET.get("start_date")
    end_date = request.GET.get("end_date")
    start_date = (
        datetime.strptime(start_date, "%Y-%m-%d").date() if start_date else None
    )
    end_date = datetime.strptime(end_date, "%Y-%m-%d").date() if end_date else None

    # Consider packets sent until midnight of the end date's day
    if end_date:
        end_date = datetime.combine(end_date, datetime.max.time())

    # Define the date range filter
    date_filter = (
        Q(packets__time_stamp__range=(start_date, end_date))
        if start_date and end_date
        else Q()
    )

    # Annotate each device with total packets and malicious packets in the date range
    annotated_devices = (
        Device.objects.filter(date_filter)
        .distinct()
        .annotate(
            total_packets=Count("packets", filter=date_filter),
            malicious_packets=Count(
                "packets", filter=date_filter & Q(packets__malicious=True)
            ),
            malicious_percentage=Case(
                When(
                    total_packets__gt=0,
                    then=(F("malicious_packets") * 100.0) / F("total_packets"),
                ),
                default=0.0,
                output_field=FloatField(),
            ),
        )
    )

    # Count total devices within the date range
    total_devices = annotated_devices.count()

    # Count devices where at least 60% of packets are malicious
    # Change threshold value here
    malicious_device_count = annotated_devices.filter(
        malicious_percentage__gte=60
    ).count()

    # Calculate non-malicious devices as total minus malicious
    non_malicious_device_count = total_devices - malicious_device_count

    # Count malicious devices grouped by group name
    malicious_by_group = (
        Group.objects.filter(
            scanners__packet__time_stamp__range=(start_date, end_date),
            scanners__packet__malicious=True,
        )
        .annotate(
            malicious_device_count=Count("scanners__packet__device", distinct=True)
        )
        .values("name", "malicious_device_count")
    )

    # Construct the response
    data = {
        "total_devices": total_devices,
        "malicious_devices": malicious_device_count,
        "non_malicious_devices": non_malicious_device_count,
        "malicious_by_group": list(malicious_by_group),
    }

    return JsonResponse(data, safe=False)


def fetch_devices(request):
    # Get query parameters for pagination
    page = int(request.GET.get("page", 1))  # Default to page 1
    per_page = int(request.GET.get("per_page", 500))

    # Get each device's latest packet by timestamp, ordered by latest timestamp first
    latest_packets = (
        Packet.objects.exclude(device_id__isnull=True)
        .values("device_id")
        .annotate(latest_timestamp=Max("time_stamp"))
        .order_by("-latest_timestamp")  # Reversed to get the latest first
    )

    # Get the details of each device along with its latest packet and related scanner and group
    devices_with_latest_packet = []
    for entry in latest_packets:
        device_id = entry["device_id"]
        latest_timestamp = entry["latest_timestamp"]

        # Get the device, latest packet, and scanner
        device = Device.objects.get(id=device_id)
        latest_packet = Packet.objects.filter(
            device=device, time_stamp=latest_timestamp
        ).first()

        # Get the scanner that scanned the latest packet
        scanner = latest_packet.scanner

        has_malicious_packet = Packet.objects.filter(
            device=device, malicious=True
        ).exists()

        # Collect relevant information
        device_data = {
            "id": device.id,
            "oui": latest_packet.oui,
            "company_id": latest_packet.company_id,
            "time_stamp": latest_packet.time_stamp,
            "scanner_name": scanner.name if scanner else None,
            "group": scanner.group.name if scanner else None,
            "malicious": has_malicious_packet,
        }

        devices_with_latest_packet.append(device_data)

    # Apply pagination
    paginator = Paginator(devices_with_latest_packet, per_page)
    current_page = paginator.get_page(page)

    # Return paginated results
    response_data = {
        "devices": list(current_page),  # Current page's devices
        "total_pages": paginator.num_pages,
        "current_page": current_page.number,
    }
    return JsonResponse(response_data, safe=False)


def fetch_pkt_data(request, device_pk):
    # Get the page number from the request (default to 1 if not provided)
    page = int(request.GET.get("page", 1))
    packets_per_page = 500

    # Get all packets for the device
    total_packets = Packet.objects.filter(device=device_pk).count()
    total_pages = (
        total_packets + packets_per_page - 1
    ) // packets_per_page  # Ceiling division

    # Calculate starting and ending index for slicing
    start_index = (page - 1) * packets_per_page
    end_index = page * packets_per_page

    # Fetch the current page's packets, ordered by most recent (latest) first
    packet_data = Packet.objects.filter(device=device_pk).order_by("-pk")[
        start_index:end_index
    ]
    serialized_packets = json.loads(serialize("json", packet_data))

    # Prepare the packet list for the response
    packet_list = [
        {"pk": packet["pk"], **packet["fields"]} for packet in serialized_packets
    ]

    # Response data includes pagination metadata
    data = {
        "packets": packet_list,
        "pagination": {
            "current_page": page,
            "total_pages": total_pages,
            "has_next": page < total_pages,
            "has_previous": page > 1,
        },
    }

    return JsonResponse(data, safe=False)


# render page views


def devices(request: HttpRequest) -> HttpResponse:
    return render(request, "devices.html")


def fetch_pkt_count(request):
    pkt_count = Packet.objects.all().count()

    data = {"pkt_count": pkt_count}

    return JsonResponse(data, safe=False)


def company_settings(request: HttpRequest) -> HttpResponse:
    return render(request, "companySettings.html")


def profile(request: HttpRequest) -> HttpResponse:
    return render(request, "profile.html")


def groups(request: HttpRequest) -> HttpResponse:
    context = {"groups": Group.objects.all()}

    return render(request, "groups.html", context=context)


def add_group(request: HttpRequest) -> HttpResponse:
    return render(request, "addGroup.html")


def activity(request: HttpRequest, group_pk) -> HttpResponse:
    context = {
        "this_group": Group.objects.get(pk=group_pk),
        "scanners": Scanner.objects.filter(group=group_pk),
    }

    return render(request, "activity.html", context=context)


def packets(request: HttpRequest, device_pk) -> HttpResponse:
    context = {
        "this_device": Device.objects.get(pk=device_pk),
    }

    return render(request, "packets.html", context=context)


def dashboard(request: HttpRequest) -> HttpResponse:
    context = {
        "groups": Group.objects.all(),
        "scanners": Scanner.objects.all(),
    }

    return render(request, "dashboard.html", context=context)


class AddSensor(View):
    def get(self, request: HttpRequest):
        context = {"groups": Group.objects.all()}
        return render(request, "addSensor.html", context=context)

    def post(self, request: HttpRequest):
        form = request.POST
        name = form["name"]
        group_pk = form["group_pk"]
        group = Group.objects.get(pk=group_pk)
        company = None
        raise NotImplemented
        # new_sensor = Scanner(name=name, group=group, company)
        # new_sensor.save()
        # return HttpResponse("Add the scanner")


# login and create account


class Register(View):
    def get(self, request: HttpRequest):
        return render(request, "register.html")

    def post(self, request: HttpRequest):
        form = request.POST
        password1 = form["password1"]
        password2 = form["password2"]
        try:
            validate_password(password1)
        except ValidationError as err:
            messages.error(request, "\n".join(err.messages))
            return render(request, "register.html")

        if not (password1 == password2):
            messages.error(request, ("Passwords do not match. Please try again."))
            return render(request, "register.html")
        email = form["email"]
        try:
            validator = EmailValidator()
            validator(email)
        except ValidationError:
            messages.error(request, ("Not a valid email. Please try again."))
            return render(request, "register.html")
        if User.objects.filter(username=email).first():
            messages.error(request, ("You are already registered. Please log in."))
            return redirect("login")
        _ = User.objects.create_user(username=email, email=email, password=password1)
        user_sign_in = authenticate(request, username=email, password=password1)
        login(request, user_sign_in)
        messages.success(
            request, ("An account has been created and you are logged in.")
        )
        return redirect("dashboard")
        return redirect("dashboard")


class Login(View):
    def get(self, request: HttpRequest):
        return render(request, "login.html")

    def post(self, request: HttpRequest):
        form = request.POST
        email = form["email"]
        password = form["password"]

        user = authenticate(request, username=email, password=password)
        print(user)
        if user is None:
            messages.error(request, ("There was error signing you in."))
            return redirect("login")
        login(request, user)
        messages.success(request, ("Login Successful"))
        return redirect("dashboard")
        return redirect("dashboard")


def logout_user(request: HttpRequest) -> HttpResponse:
    messages.success(request, ("Signed out"))
    logout(request)
    return redirect("login")
