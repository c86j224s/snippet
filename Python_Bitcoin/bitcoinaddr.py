#!/usr/bin/env python3
# -*- coding:utf-8 -*-

import ecdsa
import base58
import hashlib


# ref : https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses
# ref :https://davanum.wordpress.com/2014/03/17/generating-a-bitcoin-private-key-and-address/


def privkeytopubkey(privk):
    sk = ecdsa.SigningKey.from_string(privk, curve=ecdsa.SECP256k1)
    pubk = b'\04' + sk.verifying_key.to_string()
    return pubk


def pubkeytoaddr(pubk):
    hash1 = hashlib.sha256(pubk).digest()
    hash2 = hashlib.new('ripemd160', data=hash1).digest()
    addr = base58.b58encode_check(hash2)
    return addr


if __name__ == '__main__':
    privkey = bytes.fromhex('18E14A7B6A307F426A94F8114701E7C8E774E7F9A47E2C2035DB29A206321725')

    pubkey = privkeytopubkey(privkey)
    print('pubkey: {}'.format(pubkey.hex()))

    addr = pubkeytoaddr(pubkey)
    print('addr: {}'.format(addr))
