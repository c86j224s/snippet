#!/usr/bin/env python3 
# -*- coding: utf-8 -*-


# HISTORY:
#  2016-05-10, newly created torrent.py, added QBittorrent, TorrentKim3Net, Nas classes.
#  2016-05-11, implement QBitTorrent.getlist, getcompletedone, delete, TorrentKim3Net.copy
#  2016-05-12, add TorrentKim3Net.filtersubject


import requests
import json
from bs4 import BeautifulSoup as Soup
import sqlalchemy
from pprint import pprint
import os
import hashlib
import shutil


# TODO split downloadpolicy to local configuration file
downloadpolicy = {
	'냉장고를 부탁해': {
		'filterkeywords': ['냉장고를', '부탁해', '720p', 'WITH'],
	},
	'동네변호사 조들호': {
		'filterkeywords': ['동네', '변호사', '조들호', '720p', 'WITH'],
	},
	'마녀의 성': {
		'filterkeywords': ['마녀의', '성', 'WITH'],
	},
	'마스터 국수의 신': {
		'filterkeywords': ['국수의', '신', '720p', 'WITH'],
	},
	'굿바이 미스터 블랙': {
		'filterkeywords': ['굿바이', '미스터', '블랙', '720p', 'WITH'],
	},
	'몬스터': {
		'filterkeywords': ['몬스터', '720p', 'WITH'],
	},
	'옥중화': {
		'filterkeywords': ['옥중화', '720p', 'WITH'],
	},
	'뱀파이어 탐정': {
		'filterkeywords': ['뱀파이어', '탐정', '720p', 'WITH'],
	},
	'딴따라': {
		'filterkeywords': ['딴따라', '720p', 'WITH'],
	},
	'보컬전쟁 신의 목소리': {
		'filterkeywords': ['보컬', '전쟁', '신의', '목소리', '720p', 'WITH'],
	},
	'노래의 탄생': {
		'filterkeywords': ['노래의', '탄생', '720p', 'WITH'],
	},
}


class QBittorrent:
	addr = 'http://127.0.0.1:6600/'
	user = os.environ['qt_user']
	pwd = os.environ['qt_pwd']

	def __init__(self):
		self.cookies = None

	def __del__(self):
		pass

	def auth(self):
		r = requests.post(
			data={'username':self.user, 'password':self.pwd},
			url=self.addr + 'login'
			)
		if r.status_code != 200:
			self.cookies = None
			return False
		self.cookies = r.cookies
		return True

	def getlist(self, filter='all'):
		r = requests.get(
			cookies=self.cookies,
			url=self.addr + 'query/torrents',
			params={'filter':filter}
			)
		if r.status_code != 200:
			return []
		response = json.loads(r.content.decode('utf-8'))
		ret_list = []
		for each in response:
			ret_list += [{
				'hash':each['hash'],
				'name':each['name'],
				'progress':each['progress'],
				'state':each['state']
			}]
		return ret_list

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
		completed = getlist(filter='completed')
		if len(completed) == 0:
			return None
		return completed[0]

	def delete(self, hash):
		r = requests.post(
			cookies=self.cookies,
			data={'hashes':hash},
			url=self.addr + 'command/delete'
		)
		return True if r.status == 200 else False


class TorrentKim3Net:
	addr = 'https://torrentkim3.net/'
	res_tvdrama = 'bbs/bc.php?bo_table=torrent_tv'
	res_variety = 'bbs/bc.php?bo_table=torrent_variety'
	downloadpolicy = downloadpolicy

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

	def filtersubject(self, subject):
		for vid in self.downloadpolicy:
			matched = functools.reduce(
				lambda x, y: x and y,
				map(
					lambda keyw: 
						keyw in subject,
					self.downloadpolicy[vid]['filterkeywords']
					)
				)
			if matched:
				return True
		return False

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


if __name__ == '__main__':
	q = QBittorrent()
	q.auth()

	t = TorrentKim3Net()
	l = t.getlist_tvdrama()

	q.download(t.getmagnetfrom(l[1]['href']))