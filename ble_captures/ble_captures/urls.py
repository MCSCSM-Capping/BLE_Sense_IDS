"""
URL configuration for ble_captures project.

The `urlpatterns` list routes URLs to views. For more information please see:
    https://docs.djangoproject.com/en/5.1/topics/http/urls/
Examples:
Function views
    1. Add an import:  from my_app import views
    2. Add a URL to urlpatterns:  path('', views.home, name='home')
Class-based views
    1. Add an import:  from other_app.views import Home
    2. Add a URL to urlpatterns:  path('', Home.as_view(), name='home')
Including another URLconf
    1. Import the include() function: from django.urls import include, path
    2. Add a URL to urlpatterns:  path('blog/', include('blog.urls'))
"""

from django.contrib import admin
from django.urls import path
from client.views import *

urlpatterns = [
    path("admin/", admin.site.urls),
    path("login/", Login.as_view(), name="login"),
    path("register/", Register.as_view(), name="register"),
    path("logout_user/", logout_user, name="logout_user"),
    path("groups/", groups, name="groups"),
    path("addGroup/", add_group, name="add_group"),
    path("addSensor/", AddSensor.as_view(), name="add_sensor"),
    path("", dashboard, name="dashboard"),
    path("", dashboard, name="dashboard"),
    path("activity/<int:group_pk>", activity, name="activity"),
    path("packets/<int:device_id>", packets, name="packets"),
    path("api/fetch-data/", fetch_data, name='fetch_data'),
    path("api/fetch-devices/", fetch_devices, name='fetch_devices'),
    path("api/fetch-pkt-count/", fetch_pkt_count, name='fetch_pkt_count'),
    path("attacks/", attacks, name="attacks"),
    path("companySettings", company_settings, name="company_settings"),
    path("profile/", profile, name="profile"),
    path("devices/", devices, name="devices"),
]
