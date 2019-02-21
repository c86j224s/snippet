#!/usr/bin/env python3
# -*- coding: utf-8 -*-

'''
Dummy echo server based on binary protocol with asyncio
'''

import asyncio
import struct


async def handle_echo(reader, writer):
    while True:
        is_close = False

        length, = struct.unpack('<i', await reader.read(4))
        print('Preparing {} bytes len'.format(length))
        command, = struct.unpack('<i', await reader.read(4))
        print('Received command {}'.format(command))
        payload = await reader.read(length-4)
        print('Received {} bytes payload'.format(len(payload)))
        tid, body, = struct.unpack('<i{}s'.format(len(payload)-4), payload)
        print('Received tid = {}'.format(tid))

        if command == 2:
            is_close = True
        
        response_body = b'this is payload data'
        payload = struct.pack('<ii{}s'.format(len(response_body)+4+4), command, tid, response_body)
        writer.write(struct.pack('<i', len(payload)))
        writer.write(payload)
        await writer.drain()

        if is_close:
            print('Close connection by client request.')
            writer.close()
            break


if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    coro = asyncio.start_server(handle_echo, '127.0.0.1', 9999, loop=loop)
    server = loop.run_until_complete(coro)

    print('serving on {}'.format(server.sockets[0].getsockname()))
    
    try:
        loop.run_forever()
    except KeyboardInterrupt:
        print('key interrupt')
        pass

    server.close()
    loop.run_until_complete(server.wait_closed())
    loop.close()

