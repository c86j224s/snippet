#!/usr/bin/env python3
# -*- coding: utf8 -*-

import requests
import json
from base64 import b64encode


class BtcContext:
    def __init__ (self, rpcserver, rpcport, rpcuser, rpcpassword):
        self.rpcserver = rpcserver
        self.rpcport = rpcport
        self.rpcuser = rpcuser
        self.rpcpassword = rpcpassword
        self.rpcrequestid = 1

    def incrpcrequestid(self):
        rpcrequestid = self.rpcrequestid
        self.rpcrequestid += 1
        return rpcrequestid


def builduri(ctx):
    return 'http://{}:{}/'.format(ctx.rpcserver, ctx.rpcport)


def buildauthzheader(ctx):
    return 'Basic {}'.format(b64encode(
        ctx.rpcuser + b':' + ctx.rpcpassword
        ).decode('utf8'))


def btcrequest(ctx, method, params):
    r = requests.post(builduri(ctx),
        headers = {
            'Host' : ctx.rpcserver,
            'Connection' : 'close',
            'Content-Type' : 'application/json',
            'Authorization' : buildauthzheader(ctx)
            },
        json={
            'jsonrpc' : '1.0',
            'method' : method,
            'params' : params,
            'id' : ctx.incrpcrequestid()
            }
        )
    if r.status_code != 200:
        return None
    return r.json()


def getinfo(ctx):
    return btcrequest(ctx, 'getinfo', [])


def getnetworkinfo(ctx):
    return btcrequest(ctx, 'getnetworkinfo', [])


def getwalletinfo(ctx):
    return btcrequest(ctx, 'getwalletinfo', [])


if __name__ == '__main__':
    ctx = BtcContext(
        rpcserver='127.0.0.1', 
        rpcport = 18332,
        rpcuser = b'u1',
        rpcpassword = b'p1'
    )
    print(getinfo(ctx))
    print(getnetworkinfo(ctx))
    print(getwalletinfo(ctx))

