#!/usr/bin/env python3
# -*- coding: utf8 -*-

from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base

Base = declarative_base()

engine = create_engine('sqlite:///db.sqlite', echo=True)

class Torrent(Base):
	__tablename__ = "torrent"

	infohash = Column(String, primary_key=True)
	torrent_subject = Column(Sring)
	naspath = Column(String)
	status = Column(Integer)

	def __repr__(self):
		return "<Torrent(infohash='%s', subject='%s', naspath='%s', status='%u')>" % (
			self.infohash, self.torrent_subject, self.naspath, self.status
		)

