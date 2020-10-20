# Import Section
import logging
from werkzeug.security import generate_password_hash, check_password_hash
# 

class User:
	def __init__(self, username, password):
		self._username = username
		self._password = generate_password_hash(password)
		self._token = None
		self._buffer = None
	def set_token(self, token):
		self._token = token
	def put_message(self, message):
		self._buffer = message
	def flush(self):
		_ = self._buffer
		self._buffer = None
		return _


class Room:
	def __init__(self, name, admin):
		self.name = name
		self._admin = admin
		self._members = [admin]
	def add_member(self, user):
		self._members.append(user)
	def remove_member(self, user):
		self._members.remove(user)
	def reassign_admin(self, user):
		self._admin = user
		self._members.remove(user)
		self._members.insert(0, user)


class Chat_Engine():
	def __init__(self, port=None, user_limit=100, room_limit=100):
		self.configuration = {
			'port': port,
			'user_limit': user_limit,
			'room_limit': room_limit
		}
		self._rooms = []
		self._users = []
		self._status = True if port != None else False
	def add_user(self, user):
		self._users.append(user)
		user.connect(self)
	def remove_user(self, user):
		self._users.remove(user)
	def add_room(self, room: Room):
		if room.name not in [i.name for i in self._rooms]:
			self._rooms.append(room)
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
	def load(self, data):
		self.configuration = data['info']
		self._rooms, self._users = data['rooms'], data['users']



	





		