#!/usr/bin/env python3
# -*- coding:utf-8 -*-

import ecdsa
import base58
import hashlib

import unittest


# ref : https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses


def privkeytopubkey(privk):
    sk = ecdsa.SigningKey.from_string(privk, curve=ecdsa.SECP256k1)
    pubk = b'\04' + sk.verifying_key.to_string()
    return pubk


def pubkeytoaddr(pubk):
    hash1 = hashlib.sha256(pubk).digest()
    hash2 = hashlib.new('ripemd160', data=hash1).digest()
    addr = base58.b58encode_check(b'\x00' + hash2)
    return addr


def privkeytoaddr(privk):
    pubk = privkeytopubkey(privk)
    addr = pubkeytoaddr(pubk)
    return addr


class PrivKeyToAddrTest(unittest.TestCase):
    privkey = bytes.fromhex('18E14A7B6A307F426A94F8114701E7C8E774E7F9A47E2C2035DB29A206321725')
    pubkey = '0450863AD64A87AE8A2FE83C1AF1A8403CB53F53E486D8511DAD8A04887E5B23522CD470243453A299FA9E77237716103ABC11A1DF38855ED6F2EE187E9C582BA6'
    addr = '16UwLL9Risc3QfPqBUvKofHmBQ7wMtjvM'


    def test_privkeytopubkey(self):
        self.assertEqual(
            privkeytopubkey(self.privkey).hex().lower(), 
            self.pubkey.lower()
        )

    def test_privkeytoaddr(self):
        self.assertEqual(
            privkeytoaddr(self.privkey),
            self.addr
        )


if __name__ == '__main__':
    unittest.main()
