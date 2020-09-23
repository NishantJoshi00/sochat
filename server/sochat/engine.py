# Import Section
import logging
# 
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
	def add_room(self, room):
		self._rooms.append(room)
	def remove_room(self, room):
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





		