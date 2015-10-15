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
  print("################################################################################")
  print(str.format("{} : {}", title, url))
  print("################################################################################")
  params = url[url.find("?")+1:]
  print(params)
  conn.request("GET", "/bbs/download.php?" + params, headers={
    "Host": "torrentlee.net",
    "Connection": "keep-alive",
    "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
    "Upgrade-Insecure-Requests": "1",
    "User-Agent": "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/46.0.2490.71 Safari/537.36",
    "Referer": url,
    "Accept-Encoding": "gzip, deflate, sdch",
    "Accept-Language": "ko-KR,ko;q=0.8,en-US;q=0.6,en;q=0.4",
    "Cookie": "__cfduid=d39001f668d9d6086cc08b8f3158cd9f61444609863; PHPSESSID=slm0bintfqnkf5j6dq8i39clp3; f33d2ed86bd82d4c22123c9da444d8ab=MTQ0NDYwOTU2OA%3D%3D; 96b28b766b7e0699aa91c9ff3d890663=aHR0cDovL3RvcnJlbnRsZWUubmV0Lw%3D%3D; wcs_bt=e053f97fe46850:1444904710"

  })
  r2 = conn.getresponse()
  print(r2.getheaders())
  data2 = r2.read().decode("utf8")
  print(data2)
