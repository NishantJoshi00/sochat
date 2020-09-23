import socketserver
import threading
import sys
class Handler(socketserver.BaseRequestHandler):
	def handle(self):
		user = self.handshake()
		user.run(self.request)
	def handshake(self):
		...

class ThreadedTCPServer(socketserver.ThreadingMixIn, socketserver.TCPServer):
	pass

host, port = 'localhost', 22100
server = ThreadedTCPServer((host, port), Handler)
with server:
	server_thread = threading.Thread(target=server.serve_forever)
	server_thread.daemon = True
	server_thread.start()