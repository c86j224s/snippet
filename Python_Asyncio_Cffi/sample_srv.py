#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import asyncio
from cffi import FFI

ffi = FFI()

ffi.cdef('''
	typedef struct {
		int cmd;
		int version;
	} ProtoHelo;
''')

ffi.cdef('''
	typedef struct {
		int cmd;
		int msgLen;
		char msg[10];
	} ProtoEcho;
''')

@asyncio.coroutine
def sample_srv(reader, writer):
	print('Connected from {}'.format(writer.get_extra_info('peername')))

	helo = ffi.new('ProtoHelo[]', [(0, 20160202)])
	writer.write(bytes(ffi.buffer(helo)))
	yield from writer.drain()

	while True:
		tmpEcho = ffi.new('ProtoEcho[]', 1)
		try:
			ffi.buffer(tmpEcho)[:] = yield from reader.read(ffi.sizeof(tmpEcho))
		except ValueError as e:
			print('ValueError: {}', e.errstr)
			break

		print('Received {}, {}, {}'.format(
			tmpEcho[0].cmd,
			tmpEcho[0].msgLen,
			ffi.string(tmpEcho[0].msg).decode('utf-8')
		))

		if tmpEcho[0].cmd == 100:
			break

		writer.write(bytes(ffi.buffer(tmpEcho)))
		print('Sent echo.')

	print('Close the client socket.')
	writer.close()

loop = asyncio.get_event_loop()
coro = asyncio.start_server(sample_srv, '127.0.0.1', 8888, loop=loop)
server = loop.run_until_complete(coro)

print('Serving on {}'.format(server.sockets[0].getsockname()))
try:
	loop.run_forever()
except KeyboardInterrupt:
	pass

server.close()
loop.run_until_complete(server.wait_closed())
loop.close()