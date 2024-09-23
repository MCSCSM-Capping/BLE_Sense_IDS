from django.http import HttpRequest, HttpResponse
from django.shortcuts import render
from client.models import *
from django.views import View
from dataclasses import dataclass


def groups(request: HttpRequest) -> HttpResponse:
    context = {"groups": Group.objects.all()}

    return render(request, "groups.html", context=context)


def add_group(request: HttpRequest) -> HttpResponse:
    return render(request, "addGroup.html")


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
        new_sensor = Scanner(name=name, group=group, company)
        new_sensor.save()
        return HttpResponse("Add the scanner")


def dashboard(request: HttpRequest) -> HttpResponse:
    context = {"groups": Group.objects.all()}
    context1 = {"sensors": Scanner.objects.all()}

    return render(request, "dashboard.html", context=context)
    # how do I return the second context here? I tried changing the third argument to a dict but it didn't work
