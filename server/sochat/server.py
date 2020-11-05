import asyncio
import websockets
import random
import sys
from engine import Chat_Engine, User
import json
import os
host = "127.0.0.1"
port = 8080


runner = Chat_Engine('server-in1')
with os.popen("openssl prime -generate -bits 16") as f:
	P = int(f.read())
with os.popen("openssl prime -generate -bits 8") as f:
	Q = int(f.read())
	

async def handle(websocket: websockets.WebSocketServerProtocol, path):
	print("Connection Established")
	action = await websocket.recv()
	if action == 'login':
		user_data = await websocket.recv()
		user_data = json.loads(user_data)
		# userdata {
		# 	username: str
		# 	password: str
		# }

		usr = runner.find_user(user_data['username'])
		if not usr:
			await websocket.send("[403 Forbidden]")
			websocket.close()
		else:
			if usr.authenticate(user_data['password']):
				await usr.run(websocket)
			else:
				await websocket.send("[403 Forbidden]")
				websocket.close()


			usr.run(websocket)
	elif action == 'signup':
		user_data = await websocket.recv()
		user_data = json.loads(user_data)
		# userdata {
		# 	username: str
		# 	password: str
		# }

		usr = runner.find_user(user_data['username'])

		if usr:
			await websocket.send("[403 Forbidden]")
			websocket.close()
		else:
			new_user = User(user_data['username'], user_data['password'])
			runner.add_user(new_user)
			await new_user.run(websocket)



start_server = websockets.serve(handle, host, port)

asyncio.get_event_loop().run_until_complete(start_server)
asyncio.get_event_loop().run_forever()