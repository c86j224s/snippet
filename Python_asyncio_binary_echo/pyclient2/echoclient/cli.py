#!/usr/bin/env python3
# -*- coding: utf-8 -*-

'''
Dummy echo client based on binary protocol with asyncio
'''

import asyncio
import struct


class conn_mgr:
    def __init__(self, addr, port, asyncio_loop):
        ''' initialize object member variables '''
        # network connection information
        self.addr = addr
        self.port = port
        # asyncio streams, tasks
        self.loop = asyncio_loop
        self.reader = None
        self.writer = None
        self.read_task =  None
        # transaction map
        self.tid = 1
        self.transactions = {}

    def transactionid(self):
        ''' issue new transaction id '''
        tid = self.tid
        self.tid += 1
        return tid

    async def open_connection(self):
        ''' open connection and start packet read loop '''
        self.reader, self.writer, = await asyncio.open_connection(self.addr, self.port, loop=self.loop)
        self.read_task = self.loop.create_task(self._read_loop())

    async def _read_loop(self):
        ''' packet read loop handling response and notification messages '''
        while True:
            command, tid, message, = await self._read_message()
            if (command, tid) in self.transactions:
                self.transactions[(command, tid)].set_result(message)
                print('handled response. {}, {}, {}'.format(command, tid, message))
            else:
                print('unhandled response. {}, {}, {}'.format(command, tid, message))

    async def request(self, command, body):
        ''' request and wait response message '''
        tid = self.transactionid()
        self.transactions[(command, tid)] = self.loop.create_future()
        await self._write_message(command, tid, body)
        return await self.transactions[(command, tid)]

    def close_connection(self):
        ''' close streams and stop the packet read loop '''
        self.writer.close()
        self.reader = None
        self.writer = None
        self.read_task.cancel()

    async def _write_message(self, command, tid, body):
        ''' write a message to stream '''
        payload = struct.pack('<ii{}s'.format(len(body)+4+4), command, tid, body)
        self.writer.write(struct.pack('<i{}s'.format(len(payload)), len(payload), payload))
        await self.writer.drain()

    async def _read_message(self):
        ''' read a message from stream '''
        length, = struct.unpack('<i', await self.reader.read(4))
        command, = struct.unpack('<i', await self.reader.read(4))
        payload = await self.reader.read(length-4)
        tid, body, = struct.unpack('<i{}s'.format(len(payload)-4), payload)
        return command, tid, body


async def tcp_echo_client(loop):
    conn = conn_mgr('127.0.0.1', 9999, loop)

    await conn.open_connection()

    body = await conn.request(1, b'this is first data')

    print('Received body = {}'.format(body.decode()))

    body = await conn.request(2, b'this is second data')

    print('Received body = {}'.format(body.decode()))

    conn.close_connection()



if __name__ == '__main__':
    loop = asyncio.get_event_loop()
    loop.run_until_complete(tcp_echo_client(loop))
    loop.stop()
    loop.close()
