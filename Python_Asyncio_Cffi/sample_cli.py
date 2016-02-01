#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import asyncio
from cffi import FFI
from pprint import pprint

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
def sample_cli(loop):
	reader, writer = yield from asyncio.open_connection(
		'127.0.0.1', 8888, loop=loop
	)

	print('Connected.')

	helo = ffi.new('ProtoHelo[]', 1)
	ffi.buffer(helo)[:] = yield from reader.read(ffi.sizeof(helo))

	print('Received Helo: {}, {}'.format(
		helo[0].cmd, helo[0].version
	))

	for i in range(0, 100+1):
		sendMsg = 'msg_{}'.format(i)
		sendEcho = ffi.new('ProtoEcho[]', [(i, len(sendMsg), sendMsg.encode('utf-8'))])
		writer.write(bytes(ffi.buffer(sendEcho)))
		yield from writer.drain()

		recvEcho = ffi.new('ProtoEcho[]', 1)
		try:
			ffi.buffer(recvEcho)[:] = yield from reader.read(ffi.sizeof(recvEcho))
		except ValueError as e:
			print('ValueError: ', e)
			break

		print('Received {}, {}, {}'.format(
			recvEcho[0].cmd,
			recvEcho[0].msgLen,
			ffi.string(recvEcho[0].msg).decode('utf-8')
		))

	writer.close()


loop = asyncio.get_event_loop()
loop.run_until_complete(sample_cli(loop))
loop.close()