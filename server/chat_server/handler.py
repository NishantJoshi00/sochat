from fastapi import websockets
from pydantic import BaseModel
from fastapi.websockets import WebSocket
from typing import Dict, List
from pydantic import BaseModel
from pydantic.error_wrappers import ValidationError
import random
import threading
import shelve
import asyncio
class User(BaseModel):
    id: str
    name: str
    token: str

    def __str__(self):
        return self.name + "#" + self.id

class Message(BaseModel):
    encrypted: bool
    data: str
    from_: str
    to: str

    def dict(self, *, include: Union['AbstractSetIntStr', 'MappingIntStrAny'] = None, exclude: Union['AbstractSetIntStr', 'MappingIntStrAny'] = None, by_alias: bool = False, skip_defaults: bool = None, exclude_unset: bool = False, exclude_defaults: bool = False, exclude_none: bool = False) -> 'DictStrAny':
        di = super().dict(include=include, exclude=exclude, by_alias=by_alias, skip_defaults=skip_defaults, exclude_unset=exclude_unset, exclude_defaults=exclude_defaults, exclude_none=exclude_none)
        di['from'] = di['from_']
        del di['from_']
        return di

async def validate_user(client_cred: User, clients: List, websocket):
    if client_cred.id == "-1":
        with shelve.open("clients.dbm") as dbm:
            keys = list(dbm.keys())
            client_cred.id = "0000"
            ids = keys
            nid = client_cred.id
            while client_cred.name + '#' + nid in ids:
                nid = random.randbytes(2).hex()
            client_cred.id = nid
            dbm[client_cred.name + "#" + client_cred.id] = client_cred
        clients.append({'user': client_cred, 'messages': []})
        return client_cred
    else:
        with shelve.open('clients.dbm') as dbm:
            ids = list(dbm.keys())
            if (client_cred.name + '#' + client_cred.id) not in ids:
                await websocket.send("INVALID");
                websocket.close()
                return
            else:
                clients.append({'user': client_cred, 'messages': []})
                return client_cred


async def reciever_boi(websocket: WebSocket, clients: List[Dict], current_user: User):
    ...
    while True:
        if not websocket.open:
            break

        data = await websocket.receive_json()
        
        data['from_'] = data['from']
        del data['from']
        data = Message.parse_obj(data)

        if data.from_ != str(current_user):
            print("Invalid sender")
            continue
        
        if data.to == 'server':
            server_instruction(data, clients)
            continue
        
        [ii for ii in clients if ii['user'] == data.to][0]['message'].append(data)

def server_instruction(data: Message, clients: List[Dict]):
    key = data.data.split(' ')[0]
    if key == '/active':
        requested_users = data.data.split(' ')[1:]
        msg = ''
        for ii in clients:
            if str(ii['user']) in requested_users:
                msg += str(ii['user']) + ': active'
        new_data = Message(encrypted=False, data=msg, from_='server', to=data.from_)
        [ii for ii in clients if ii['user'] == data.to][0]['message'].append(new_data)
                


async def sender_boi(websocket: WebSocket, clients: List[Dict], current_user: User):
    while True:
        if not websocket.open:
            break
        msgs = [ii for ii in clients if str(ii['user']) == str(current_user)][0]['messages']
        while len(msgs) > 0:
            await websocket.send_json(msgs.pop(0).dict())
        await asyncio.sleep(.1)


async def websocket_handler(websocket: WebSocket, clients: List[Dict]):
    client_cred = await websocket.receive_json()
    try:
        client_cred = User.parse_obj(client_cred)
    except ValidationError as e:
        print("Error in {}: {}".format(websocket.client, e))
        await websocket.close(300)
        return
    except Exception as e:
        print(f"Invalid data: {e}")
        await websocket.close()
        return
    print(client_cred)
    new_creds = await validate_user(client_cred, clients, websocket);
    if new_creds == None:
        return
    await websocket.send_json(new_creds.dict())

    loop = asyncio.get_event_loop()
    loop.create_task(sender_boi(websocket, clients, client_cred))
    loop.run_until_complete(reciever_boi(websocket, clients, client_cred))
    loop.close()
        
    