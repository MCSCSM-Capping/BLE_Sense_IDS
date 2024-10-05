from django.http import HttpRequest, HttpResponse
from django.shortcuts import render
from client.models import *
from django.views import View
from dataclasses import dataclass
from django.http import JsonResponse

def fetch_data(request):

    group_data = Group.objects.all().values()
    scanner_data = Scanner.objects.all().values()
    packet_data = Packet.objects.all().values()

    group_list  = list(group_data)
    scanner_list = list(scanner_data)
    packet_list = list(packet_data)

    data = {'groups': group_list, 'scanner': scanner_list, 'packets': packet_list}

    return JsonResponse(data, safe=False)


def groups(request: HttpRequest) -> HttpResponse:
    
    context = {"groups": Group.objects.all()}

    return render(request, "groups.html", context=context)


def add_group(request: HttpRequest) -> HttpResponse:
    return render(request, "addGroup.html")

def activity(request: HttpRequest, group_pk) -> HttpResponse:

    context = {"this_group": Group.objects.get(pk=group_pk)}
    
    return render(request, "activity.html", context = context)

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
        #new_sensor = Scanner(name=name, group=group, company)
        #new_sensor.save()
        #return HttpResponse("Add the scanner")

def dashboard(request: HttpRequest) -> HttpResponse:

    context = {"groups": Group.objects.all(), "sensors": Scanner.objects.all()}

    return render(request, "dashboard.html", context=context)
