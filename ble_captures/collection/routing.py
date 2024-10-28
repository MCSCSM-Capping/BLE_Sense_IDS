from collection.consumers import WebsocketConsumer
from django.urls import path

websocket_urlpatterns = [
    # Chat room websocket
    path(
        "ws/packet_collection",
        WebsocketConsumer.as_asgi(),  # pyright: ignore
        name="packet_collection",  # pyright: ignore
    ),
]
