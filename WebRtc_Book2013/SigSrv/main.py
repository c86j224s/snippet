#!/usr/bin/env python3
# -*- coding: utf-8 -*-


# flask + socketio : https://yumere.tistory.com/53


import os
from flask import Flask, render_template, session
from flask_socketio import SocketIO, emit


app = Flask(__name__)
app.secret_key = "secret"
app.config['TEMPLATES_AUTO_RELOAD'] = True

socketio = SocketIO(app)


user_count = 1


@app.before_request
def before_request():
    global user_count
    if 'session' in session and 'username' in session:
        pass
    else:
        session['session'] = os.urandom(24)
        session['username'] = 'user#'+str(session['session'])
        user_count += 1


@app.route('/')
def index():
    return render_template('index.html')


@socketio.on('connect', namespace='/mynamespace')
def connect():
    emit('chat_response', {'data': 'Connected', 'username': session['username']}, broadcast=True)


@socketio.on('disconnect', namespace='/mynamespace')
def disconnect():
    global user_count
    session.clear()
    user_count -= 1
    print('disconnected')


@socketio.on('chat', namespace='/mynamespace')
def chat(message):
    emit('chat_response', {'data': message['data'], 'username': session['username']}, broadcast=True)


if __name__ == '__main__':
    socketio.run(app)