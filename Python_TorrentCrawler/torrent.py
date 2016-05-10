#!/usr/bin/env python3 
# -*- coding: utf-8 -*-


# HISTORY:
#  2016-05-10, newly created torrent.py, added QBittorrent, TorrentKim3Net, Nas classes.



import requests
import json
from bs4 import BeautifulSoup as Soup
from pprint import pprint
import os


downloadpolicy = {
	'냉장고를 ': {title: '냉장고를 부탁해', filterkeywords: ['720p', 'WITH']},
	'조들호': {title: '동네변호사 조들호', filterkeywords: ['720p', 'WITH']},
	'마녀의': {title: '마녀의 성', filterkeywords: ['WITH']},
}


class QBittorrent:
	addr = 'http://127.0.0.1:6600/'

	def __init__(self):
		self.cookies = None

	def __del__(self):
		pass

	def auth(self):
		r = requests.post(
			data={'username':'admin', 'password':'asdfdddd1234'},
			url=self.addr + 'login'
			)
		if r.status_code != 200:
			self.cookies = None
			return False
		self.cookies = r.cookies
		return True

	def getlist(self):
		r = requests.get(
			cookies=self.cookies,
			url=self.addr + 'query/torrents'
			)
		if r.status_code != 200:
			return None
		return r.json()

	def download(self, magnet):
		r = requests.post(
			cookies=self.cookies,
			data={'urls':magnet},
			url=self.addr + 'command/download'
			)
		if r.status_code != 200:
			return False
		return True

	def getcompletedone(self):
		# TODO return 1 completed torrent hash and files list
		pass

	def delete(self, hash):
		# TODO delete torrent and files by torrent hash
		pass



class TorrentKim3Net:
	addr = 'https://torrentkim3.net/'
	res_tvdrama = 'bbs/bc.php?bo_table=torrent_tv'
	res_variety = 'bbs/bc.php?bo_table=torrent_variety'

	def __init__(self):
		pass

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

	def getlist_tvdrama(self):
		return self.getlist(self.res_tvdrama)

	def getlist_variety(self):
		return self.getlist(self.res_variety)

	def getmagnetfrom(self, articlehref):
		url = self.addr + articlehref.replace('../', '')
		r = requests.get(url=url)
		soup = Soup(r.content.decode('utf-8'), 'html.parser')
		try:
			f_list = soup.find(id='f_list')
			magnet = f_list.next_sibling.next_sibling.next_sibling.next_sibling['value']
			return magnet
		except:
			return None
		return None


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
		# TODO check md5 checksum of srcpath
		# TODO copy from srcpath to dstpath
		# TODO check md5 checksum of dstpath
		pass


if __name__ == '__main__':
	q = QBittorrent()
	q.auth()

	t = TorrentKim3Net()
	l = t.getlist_tvdrama()

	q.download(t.getmagnetfrom(l[1]['href']))