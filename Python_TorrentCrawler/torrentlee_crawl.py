#!/usr/bin/env python3
# -*- coding:utf-8 -*-

# refer to https://beautiful-soup-4.readthedocs.org/en/latest

import http.client
from bs4 import BeautifulSoup
from pprint import pprint

conn = http.client.HTTPConnection("torrentlee.net")
conn.request("GET", "/bbs/board.php?bo_table=kor_ent")
r1 = conn.getresponse()
data1 = r1.read()
soup = BeautifulSoup(data1, "html.parser")
mw_index = soup.find(id="fboardlist")
#print(mw_index.prettify())
tds = mw_index.find_all(class_="mw_basic_list_subject")
for each_td in tds:
  url = each_td.a["href"]
  title = each_td.a.span.string
  if "720p-WITH" not in title and "720P-WITH" not in title:
    continue
  print(str.format("{} : {}", title, url))
