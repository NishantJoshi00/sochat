# import socketserver
# import threading
# import random
# import sys
# from .engine import Chat_Engine, User
# import os

# runner = Chat_Engine();
# with os.popen("openssl prime -generate -bits 16") as f:
# 	P = int(f.read())
# with os.popen("openssl prime -generate -bits 8") as f:
# 	Q = int(f.read())
# class Handler(socketserver.BaseRequestHandler):
# 	def handle(self):
# 		token = self.handshake()
# 		user.run()
# 	def handshake(self):
# 		_t = self.request.recv(1)
# 		# 1. Sending the public key to client
# 		self.request.sendall(P.to_bytes(2, byteorder='little'))
# 		self.request.sendall(Q.to_bytes(1, byteorder='little'))
# 		with os.popen("openssl prime -generate -bits 8") as f:
# 			_token = int(f.read())
# 		send_key = (Q ** _token) % P
# 		self.request.sendall(send_key.to_bytes(2, byteorder='little'))
# 		recieve_key = int.from_bytes(self.request.recv(2), 'little')
# 		_token = (recieve_key ** _token) % P
# 		if _t == b'n':
# 			...
# 		elif _t == b'e':
# 			...

# 		return _token
# 	def encrypt(data, token):
# 		return data


		
		
		

# class ThreadedTCPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
# 	pass

# host, port = 'localhost', 22100
# server = ThreadedTCPServer((host, port), Handler)
# with server:
# 	server_thread = threading.Thread(target=server.serve_forever)
# 	server_thread.daemon = True
# 	server_thread.start()

"""
Sorry for the mess
"""

import .server
