from django.http import HttpRequest, HttpResponse
from django.shortcuts import redirect, render
from client.models import *
from django.views import View
from dataclasses import dataclass
from django.http import JsonResponse
from django.core.validators import EmailValidator
from django.contrib.auth.password_validation import validate_password
from django.contrib.auth import authenticate, login, logout
from django.core.exceptions import ValidationError
from django.contrib import messages
from django.contrib.auth.models import User
from channels.generic.websocket import WebsocketConsumer


class SendPacketsSocket(WebsocketConsumer):
    def connect(self):
        self.accept()

    def disconnect(self, close_code):  # pyright: ignore
        pass

    def receive(self, bytes_data):  # pyright: ignore
        print(bytes_data)
