#!/usr/bin/env python
# -*- coding: cp949 -*-

from http.server import HTTPServer, BaseHTTPRequestHandler

class custom_handler(BaseHTTPRequestHandler):
	def do_GET(self):
		self.send_response(200)
		self.send_header('Subject', self.headers['Subject'])
		self.send_header('Connection', 'keep-alive')
		self.end_headers()
		

if __name__ == '__main__':
	try:
		httpd = HTTPServer(('', 8080), custom_handler)
		httpd.serve_forever()
	except:
		httpd.socket.close()

