from collection.consumers import SendPacketsSocket
from django.urls import path

websocket_urlpatterns = [
    # Chat room websocket
    path(
        "ws/packet_collection",
        SendPacketsSocket.as_asgi(),  # pyright: ignore
        name="packet_collection",  # pyright: ignore
    ),
]
