// TorrentCrawler.cpp : 콘솔 응용 프로그램에 대한 진입점을 정의합니다.
//

#include "stdafx.h"

#include <cpprest/http_client.h>
#include <cpprest/filestream.h>
#include <iostream>

using namespace utility;
using namespace web;
using namespace web::http;
using namespace web::http::client;
using namespace concurrency::streams;

int bing_request()
{
	auto fileStream = std::make_shared<ostream>();

	// Open stream to output file.
	pplx::task<void> requestTask = fstream::open_ostream(U("results.html")).then([=](ostream outFile)
	{
		*fileStream = outFile;

		// Create http_client to send the request.
		http_client client(U("http://www.bing.com/"));

		// Build request URI and start the request.
		uri_builder builder(U("/search"));
		builder.append_query(U("q"), U("Casablanca CodePlex"));
		return client.request(methods::GET, builder.to_string());
	}).then([=](http_response response)
	{
		printf("Received response status code:%u\n", response.status_code());

		// Write response body into the file.
		return response.body().read_to_end(fileStream->streambuf());
	}).then([=](size_t)
	{
		return fileStream->close();
	});

	try {
		requestTask.wait();
	}
	catch (const std::exception & e) {
		printf("Error exception:%s\n", e.what());
	}

    return 0;
}

int sample() {
	pplx::task<void> t = pplx::task<int>([=]() {
		return 10;
	}).then([=](int n) {
		printf("first then = %u\n", n);
		return 20;
	}).then([=](int n) {
		printf("second then = %u\n", n);
	});

	try {
		t.wait();
	}
	catch (const std::exception & e) {
		printf("Error exception:%s\n", e.what());
	}

	return 0;
}

#define interface struct

interface ITorrentClient {

	

	virtual ~ITorrentClient() {}

	virtual bool login(string_t id, string_t pwd) abstract;

	virtual std::vector<string_t> torrents(string_t filter) abstract;

};

class QBitTorrent : 
	public ITorrentClient 
{
	http_client m_client;
	string_t m_cookie;

public:

	QBitTorrent(string_t host) :
		m_client(host)
	{
	}


	bool login(string_t id, string_t pwd) override {
		string_t body(U("username=") + id + U("&password=") + pwd);
		
		pplx::task<bool> t = pplx::task<void>([=]() {}).then([=]() {
			http_request req(methods::POST);
			req.headers().set_content_type(U("application/x-www-form-urlencoded"));
			req.set_request_uri(U("login"));
			req.set_body(body);

			return m_client.request(req);
		}).then([=](http_response resp) {
			if (resp.status_code() != 200)
				return false;
			
			m_cookie = resp.headers()[U("Set-Cookie")];
			return true;
		});
		
		return t.get();
	}


	std::vector<string_t> torrents(string_t filter) {
		pplx::task<std::vector<string_t>> t = pplx::task<void>([=]() {}).then([=]() {
			http_request req(methods::GET);
			req.headers().add(U("Cookie"), m_cookie);
			req.set_request_uri(U("query/torrents"));

			return m_client.request(req);
		}).then([=](http_response resp) {
			std::vector<string_t> ret;

			

			return ret;
		});
	}
};

int main() {
	QBitTorrent qb(U("http://localhost:6600/"));
	bool login_res = qb.login(U("admin"), U("asdfdddd"));
	std::cout << login_res << std::endl;


	return 0;
}