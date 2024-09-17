from django.http import HttpRequest, HttpResponse
from django.shortcuts import render
from dataclasses import dataclass


@dataclass
class Group:
    name: str
    code: str
    city: str
    state: str
    sensor: str

@dataclass
class Sensor:
    name: str
    code: str
    ID: str

sensor1 = Sensor(
    name="Group A", code="xxxx", ID="24001"
)

sensor2 = Sensor(
    name="Group A", code="xxxx", ID="24003"
)

sensor3 = Sensor(
    name="Group B", code="54321", ID="777"
)

group1 = Group(
        name="Group A", code="xxxx", city="New York", state="NY", sensor="Sensor1"
    )
group2 = Group(
        name="Group B", code="54321", city="Los Angeles", state="CA", sensor="Sensor2"
    )
group3 = Group(
        name="Group C", code="98765", city="Chicago", state="IL", sensor="Sensor3"
    )    


def groups(request: HttpRequest) -> HttpResponse:

    context = {"groups": [group1, group2, group3]}

    return render(request, "groups.html", context=context)

def add_group(request:HttpRequest) -> HttpResponse:
    return render(request, "addGroup.html")
    #do I need to add some more code here?

def add_sensor(request:HttpRequest) -> HttpResponse:

    context = {"groups": [group1, group2, group3]}

    return render(request,"addSensor.html", context=context)

def dashboard(request:HttpRequest) -> HttpResponse:

    context = {"groups": [group1, group2, group3]}
    context1 = {"sensors": [sensor1,sensor2, sensor3]}

    return render(request, "dashboard.html", context=context)