from fastapi import websockets
from pydantic import BaseModel
from fastapi.websockets import WebSocket, WebSocketDisconnect
from starlette.websockets import WebSocketState
from typing import Dict, List
from pydantic import BaseModel
from pydantic.error_wrappers import ValidationError
import random
import threading
from typing import Union
import shelve
import asyncio
class User(BaseModel):
    id: str
    name: str
    token: str

    def __str__(self):
        return self.name + "#" + self.id
    
    def debug(self):
        return f'{self.name}#{self.id} : {self.token}'
    
    # compare to string
    def __eq__(self, other):
        return str(self) == other

class Message(BaseModel):
    encrypted: bool
    data: str
    from_: str
    to: str


    def dict(self, *, include= None, exclude= None, by_alias: bool = False, skip_defaults: bool = None, exclude_unset: bool = False, exclude_defaults: bool = False, exclude_none: bool = False) -> dict:
        di = super().dict(include=include, exclude=exclude, by_alias=by_alias, skip_defaults=skip_defaults, exclude_unset=exclude_unset, exclude_defaults=exclude_defaults, exclude_none=exclude_none)
        di['from'] = di['from_']
        del di['from_']
        return di



def disconnect_user(user, clients):
    index = [i for i, ii in enumerate(clients) if ii['user'] == user]
    if len(index) == 0:
        return
    index = index[0]
    clients.pop(index)


def send_message(clients, message):
    sent = len([client['messages'].append(message) for client in clients if client['user'] == message.to])
    if sent == 0:
        send_message(clients, Message(encrypted=False, data='{} is not online'.format(message.to), from_='server', to=message.from_))

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
                if len([False for client in clients if client['user'] == client_cred]) != 0:
                    await websocket.send("INVALID");
                    websocket.close()
                    return
                clients.append({'user': client_cred, 'messages': []})
                return client_cred


async def reciever_boi(websocket: WebSocket, clients: List[Dict], current_user: User):
    try:
        while True:
            # print(websocket.application_state, websocket.client_state)
            
            # if not websocket.open:
            #     break
            if not (websocket.application_state == WebSocketState.CONNECTED and websocket.client_state == WebSocketState.CONNECTED):
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
            
            # [ii for ii in clients if ii['user'] == data.to][0]['messages'].append(data)
            send_message(clients, data)

    except Exception as e:
        # print(e)
        print("{} disconnected".format(current_user))
        disconnect_user(current_user, clients)


def server_instruction(data: Message, clients: List[Dict]):
    print(data)
    key = data.data.split(' ')[0]
    if key == '/active':
        requested_users = [name for name in data.data.split(' ')[1:]]
        msg = ''
        for ii in clients:
            print(ii['user'], requested_users)
            if str(ii['user']) in requested_users:
                msg += str(ii['user']) + ': active'
        new_data = Message(encrypted=False, data=msg, from_='server', to=data.from_)
        # [ii for ii in clients if ii['user'] == new_data.to][0]['messages'].append(new_data)
        send_message(clients, new_data)
    elif key == '/online':
        user_info = data.data.split(' ')[1]
        from_user = data.from_

        
        if user_info == from_user:
            msg = 'You are online'
        else:
            msg = 'Something is wrong in your configuration'
        new_data = Message(encrypted=False, data=msg, from_='server', to=data.from_)
        
        # [ii for ii in clients if ii['user'] == new_data.to][0]['messages'].append(new_data)
        send_message(clients, new_data)

                


async def sender_boi(websocket: WebSocket, clients: List[Dict], current_user: User):
    while True:
        # print(websocket.application_state, websocket.client_state, websocket.state)
        # if not websocket.open:
        #     break
        if not (websocket.application_state == WebSocketState.CONNECTED and websocket.client_state == WebSocketState.CONNECTED):
            break
        
        msgs = [ii for ii in clients if str(ii['user']) == str(current_user)]
        if len(msgs) == 0:
            continue
        msgs = msgs[0]['messages']

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
    asyncio.create_task(sender_boi(websocket, clients, client_cred))
    await reciever_boi(websocket, clients, client_cred)
    
        
    