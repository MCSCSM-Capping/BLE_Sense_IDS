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


def groups(request: HttpRequest) -> HttpResponse:
    group1 = Group(
        name="Group A", code="12345", city="New York", state="NY", sensor="Sensor1"
    )
    group2 = Group(
        name="Group B", code="54321", city="Los Angeles", state="CA", sensor="Sensor2"
    )
    group3 = Group(
        name="Group C", code="98765", city="Chicago", state="IL", sensor="Sensor3"
    )

    context = {"groups": [group1, group2, group3]}

    return render(request, "groups.html", context=context)
