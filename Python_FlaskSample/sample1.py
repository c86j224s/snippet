#!/usr/bin/env/python3 
# -*- coding: utf-8 -*-

import json

from flask import Flask
app = Flask(__name__)

@app.route('/')
def hello():
	return json.dumps(
		{
			"elem1": "value1",
			"arr": [1,2,"3",4,5]
		}
	)

if __name__ == "__main__":
	app.run()
