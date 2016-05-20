#!/usr/bin/env python3 
# -*- encoding:utf8 -*-


# HISTORY:
#  2016-05-10, newly created torrent.py, added QBittorrent, TorrentKim3Net, Nas classes.
#  2016-05-11, implement QBitTorrent.getlist, getcompletedone, delete, TorrentKim3Net.copy
#  2016-05-12, add TorrentKim3Net.filtersubject


import requests
import json
from bs4 import BeautifulSoup as Soup
import sqlalchemy
from qbittorrent import Client
from pprint import pprint
import os
import hashlib
import functools
import shutil
import sys
import codecs
import time


class TorrentKim3Net:
	def __init__(self, crawlinfo, downloadpolicy):
		self.addr = crawlinfo['addr']
		self.res_tvdrama = crawlinfo['tvdrama']
		self.res_variety = crawlinfo['variety']
		self.downloadpolicy = downloadpolicy

	def __del__(self):
		pass

	def getlist(self, respath):
		r = requests.get(url=self.addr + respath)
		soup = Soup(r.content.decode('utf-8'), 'html.parser')
		rows = soup.find(id='main_body').find('table', 'board_list').find_all('tr')
		links = []
		for row in rows:
			try:
				if ('style' in row.attrs and
					row['style'] == 'display:none'
					):
					contineu
				link = {}
				link['num'] = int(row.find('td', 'num').text)
				link['href'] = row.find('td', 'subject').a['href']
				link['subject'] = row.find('td', 'subject').a.text
				links += [link]
			except:
				pass
		return links

	def getlist_tvdrama(self, page=1):
		return self.getlist(self.res_tvdrama + str(page))

	def getlist_variety(self, page=1):
		return self.getlist(self.res_variety + str(page))

	def filtersubject(self, subject):
		for vid in self.downloadpolicy:
			for filterkeywords in self.downloadpolicy[vid]['filterkeywords']:
				matched = functools.reduce(
					lambda x, y: x and y,
					map(
						lambda keyw: 
							keyw in subject,
						filterkeywords
						)
					)
				if matched:
					return True
		return False

	def getmagnetfrom(self, articlehref):
		url = self.addr + articlehref.replace('../', '')
		try:
			r = requests.get(url=url)
			soup = Soup(r.content.decode('utf8'), 'html.parser')
			f_list = soup.find(id='f_list')
			magnet = f_list.next_sibling.next_sibling.next_sibling.next_sibling['value']
			return magnet
		except:
			return None
		return None


class DownloadDb:
	STATUS_ADDED = 1
	STATUS_DOWNLOADED = 2
	STATUS_MOVED = 3

	def __init__(self, dbpath):
		self.dbpath = dbpath
		self.needwrite = False
		self.sync()

	def added(self, magnet):
		if magnet not in self.db:
			self.db[magnet] = self.STATUS_ADDED
			self.needwrite = True

	def isadded(self, magnet):
		if magnet in self.db:
			return True
		return False

	def downloaded(self, magnet):
		if magnet not in self.db or self.db[magnet] == self.STATUS_ADDED:
			self.db[magnet] = self.STATUS_DOWNLOADED
			self.needwrite = True

	def isdownloaded(self, magnet):
		if (magnet in self.db and
			self.db[magnet] in (self.STATUS_DOWNLOADED, self.STATUS_MOVED)
		):
			return True
		return False

	def moved(self, magnet):
		if magnet not in self.db or self.db[magnet] == self.STATUS_DOWNLOADED:
			self.db[magnet] = self.STATUS_MOVED
			self.needwrite = True

	def ismoved(self, magnet):
		if (magnet in self.db and self.db[magnet] == self.STATUS_MOVED):
			return True
		return False

	def sync(self):
		if self.needwrite:
			with codecs.open(self.dbpath, 'w', encoding='utf8') as f:
				f.write(json.dumps(self.db))
			self.needwrite = False
		else:
			with codecs.open(self.dbpath, 'r', encoding='utf8') as f:
				self.db = json.loads(f.read())


class Nas:
	def __init__(self):
		self.uncpath = os.environ['nas_path']
		self.user = os.environ['nas_user']
		self.pwd = os.environ['nas_pwd']

	def __del__(self):
		pass

	def auth(self):
		os.system('net use {uncpath} /user:{user} {pwd}'.format(uncpath=self.uncpath, user=self.user, pwd=self.pwd))
		# TODO check return value

	def copy(self, srcpath, dstpath):
		try:
			srcchecksum = None
			dstchecksum = None
			with open(srcpath, 'r') as f:
				srcchecksum = hashlib.md5(f.read()).hexdigest()
			shutil.copyfile(srcpath, dstpath)
			with open(srcpath, 'r') as f:
				dstchecksum = hashlib.md5(f.read()).hexdigest()
			if srcchecksum != dstchecksum:
				return False
		except:
			return False
		return True


def main():
	# load config
	with codecs.open('config.json', 'r', encoding='utf8') as f:
		cfg = json.loads(f.read())
	torrentcfg = cfg['torrent']
	policycfg = cfg['downloadpolicy']
	dbcfg = cfg['db']
	crawlcfg = cfg['crawl']

	# init db 
	db = DownloadDb(dbcfg)

	# get qbittorrent connection
	q = Client(torrentcfg['addr'])
	errmsg = q.login(torrentcfg['user'], torrentcfg['pwd'])
	if errmsg:
		print('Torrent server ' + errmsg, file=sys.stderr)
	
	# crawl
	t = TorrentKim3Net(
		crawlinfo = crawlcfg['torrentkim3.net'],
		downloadpolicy = policycfg
	)
	l = []
	for i in range(1, 3+1):
		l += t.getlist_tvdrama(page=i)
		l += t.getlist_variety(page=i)
	
	print('\n########## Crawl torrentkim3.net')
	for each in l:
		subj = each['subject']
		matched = t.filtersubject(subj)
		if not matched:
			print('not matched : ' + subj)
			continue

		magnet = t.getmagnetfrom(each['href'])
		if not magnet:
			print('failed to get magnet : ' + subj)
			continue

		if db.isadded(magnet):
			print('already added : ' + subj)
		else:
			q.download_from_link(magnet)
			print('added : '+ subj)
			db.added(magnet)
	
	db.sync()

	time.sleep(1)

	# check qbittorrent status

	print('\n########## QBittorrent Status')
	for each in q.torrents():
		progress = each['progress']
		percent = str(100 * progress) + ' %'
		name = each['name']
		magnet = 'magnet:?xt=urn:btih:' + each['hash'].lower()
		print(percent + ' | ' + name + ' | ' + magnet)
		if progress == 1 and not db.isdownloaded(magnet):
			db.downloaded(magnet)

	db.sync()


if __name__ == '__main__':
	while True:
		print('\n#################### [ Running at ' + time.ctime() + ' ] ##########')
		main()
		time.sleep(300)