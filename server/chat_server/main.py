# from typing import Dict, Optional
from fastapi import FastAPI
from pydantic import BaseModel
import psutil
import handler

from fastapi.websockets import WebSocket
app = FastAPI()

clients = []

@app.get("/")
async def root():
    return { 'status_code': 200 }

@app.get("/health")
async def health():
    return {
        'cpu': psutil.cpu_percent(),
        'memory': psutil.virtual_memory(),
    }


@app.websocket_route("/ws")
async def ws_route(websocket: WebSocket):
    await websocket.accept()
    await handler.websocket_handler(websocket, clients)
        
