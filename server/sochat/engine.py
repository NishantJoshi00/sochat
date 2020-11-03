# Import Section
import logging
import asyncio
import websockets
from werkzeug.security import generate_password_hash, check_password_hash
from typing import Union
import json

class User:
	def __init__(self, username: str, password: str):
		self._username = username
		self._password = generate_password_hash(password)
		# self._token = None
		self._buffer = []
		self.chatengine = None
	# def set_token(self, token):
	# 	self._token = token
	def put_message(self, message):
		self._buffer.append(message)
	def flush(self):
		_ = self._buffer
		self._buffer = None
		return _
	def authenticate(self, password: str):
		return check_password_hash(self._password, password)
	async def run(self, websoc: websockets.WebSocketServerProtocol):
		async for message in websoc:
			if message.startswith("{"):
				self.chatengine.route((message))
			else:
				break
			if len(self._buffer) != 0:
				await websoc.send(json.dumps(self.flush()))

				


class Room:
	def __init__(self, name: str, admin: User):
		self.name = name
		self._admin = admin
		self._members = [admin]
	def add_member(self, user: User):
		# user.add_room(self)
		self._members.append(user)
	def remove_member(self, user: User):
		# user.exit_room(self)
		self._members.remove(user)
	def reassign_admin(self, user: User):
		self._admin = user
		self._members.remove(user)
		self._members.insert(0, user)
	def put_message(self, message):
		[i.put_message(message) for i in self._members]


class Chat_Engine():
	def __init__(self, name, port=None, user_limit=100, room_limit=100):
		self.name = name
		self.configuration = {
			'port': port,
			'user_limit': user_limit,
			'room_limit': room_limit
		}
		self._rooms = {}
		self._users = {}
		self._status = True if port != None else False
	def add_user(self, user: User):
		user.chatengine = self
		self._users[user.name] = user
		# user.connect(self)
	def remove_user(self, user: User):
		del self._users[user.name]
	def find_user(self, uname: str):
		if uname in self._users:
			return self._users[uname]
		else:
			return False

	def add_room(self, room: Room):
		if room.name not in self._rooms:
			self._rooms[room.name] = room
			return 0
		else:
			return -1
	def remove_room(self, room: Room):
		self._rooms.remove(room)
	def dump(self):
		data = {
			'info': self.configuration,
			'rooms': [i.dump() for i in self._rooms],
			'users': [i.dump() for i in self._users]
		}
		return data
	def load(self, data: dict):
		self.configuration = data['info']
		self._rooms, self._users = data['rooms'], data['users']
	def parse_message(self, message):
		...
		return message
	def route(self, message):
		message = self.parse_message(message)
		recv = message['reciever']
		recv.put_message(message)



		



	
# Message
# {
# 	sender: User
# 	reciever: User | Room
# 	content: str | bytes
# 	datetime: {
#		sender: Datetime()
#		server: Datetime()
# 	}
# 	hash: str
# }


		