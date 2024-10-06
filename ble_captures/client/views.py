from django.http import HttpRequest, HttpResponse
from django.shortcuts import redirect, render
from client.models import *
from django.views import View
from dataclasses import dataclass
from django.core.validators import EmailValidator
from django.contrib.auth.password_validation import validate_password
from django.contrib.auth import authenticate, login, logout
from django.core.exceptions import ValidationError
from django.contrib import messages
from django.contrib.auth.models import User


def groups(request: HttpRequest) -> HttpResponse:
    context = {"groups": Group.objects.all()}

    return render(request, "groups.html", context=context)


def add_group(request: HttpRequest) -> HttpResponse:
    return render(request, "addGroup.html")


def home(request: HttpRequest) -> HttpResponse:
    return render(request, "home.html")


def activity(request: HttpRequest, group_pk) -> HttpResponse:
    context = {"this_group": Group.objects.get(pk=group_pk)}

    return render(request, "activity.html", context=context)


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


def dashboard(request: HttpRequest) -> HttpResponse:
    context = {
        "groups": Group.objects.all(),
        "sensors": Scanner.objects.all(),
        "attacks": attacks,
    }

    return render(request, "dashboard.html", context=context)


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
            messages.error(request, err.message)
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
        return redirect("home")


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
        return redirect("home")


def logout_user(request: HttpRequest) -> HttpResponse:
    messages.success(request, ("Signed out"))
    logout(request)
    return redirect("login")
