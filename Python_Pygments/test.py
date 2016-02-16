#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys
from pprint import pprint
from pygments import highlight
from pygments.lexers import CppLexer
from pygments.formatter import Formatter
from pygments.token import Token

class MyFormatter(Formatter):
	def format(self, tokensource, outfile):
		lineno = 1
		for ttype, value in tokensource:
			print("##########" + str(lineno))
			pprint(ttype, stream=outfile)
			pprint(value, stream=outfile)
			if ttype in Token.Text and value == '\n':
				lineno += 1
			

if __name__ == "__main__":
	with open("sample.cpp", "r") as f:
		highlight(f.read(), CppLexer(), MyFormatter(full=True), sys.stdout)
