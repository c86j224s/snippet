// WebRtcPeerConnectionTutorial.cpp : 이 파일에는 'main' 함수가 포함됩니다. 거기서 프로그램 실행이 시작되고 종료됩니다.
//

#include <array>
#include <iostream>
#include <vector>
//#include <windows.h>

#define WEBRTC_WIN 1
#include "rtc_base/win32_socket_init.h"
#include "rtc_base/win32_socket_server.h"
#include "rtc_base/net_helpers.h"


#pragma comment(lib, "obj/webrtc.lib")
//#pragma comment(lib, "obj/rtc_base/rtc_base.lib")
#pragma comment(lib, "ws2_32.lib")


class ControlClient : public sigslot::has_slots<>, public rtc::MessageHandler {
	rtc::SocketAddress m_serveraddr{};
	std::unique_ptr<rtc::AsyncResolver> m_resolver{};
	std::unique_ptr<rtc::AsyncSocket> m_socket{};
	std::vector<char> m_reading_buf;

protected:
	// inherited from rtc::MessageHandler
	void OnMessage(rtc::Message* msg) override {

	}

	void OnResolved(rtc::AsyncResolverInterface* resolver) {
		std::cout << "on resolved" << std::endl;
		if (m_resolver->GetError() != 0) {
			m_resolver->Destroy(false);
			m_resolver.reset();
			RaiseException(0, 0, 0, nullptr);
		}
		else {
			m_serveraddr = m_resolver->address();
			DoConnect();
		}
	}

public:
	void DoConnect() {
		auto * s = new rtc::Win32Socket{};
		
		s->CreateT(m_serveraddr.family(), SOCK_STREAM);
		m_socket.reset(s);

		s->SignalCloseEvent.connect(this, &ControlClient::OnClosed);
		s->SignalConnectEvent.connect(this, &ControlClient::OnConnected);
		s->SignalReadEvent.connect(this, &ControlClient::OnRead);
		s->SignalWriteEvent.connect(this, &ControlClient::OnWrite);

		int err = m_socket->Connect(m_serveraddr);
		if (err == SOCKET_ERROR) {
			std::cout << "connect error" << std::endl;
			m_socket->Close();
			m_socket.reset();
			return;
		}
		std::cout << "connecting" << std::endl;
	}


	void OnClosed(rtc::AsyncSocket* socket, int err) {
		std::cout << "on closed" << std::endl;

		socket->Close();
		if (err != WSAECONNREFUSED) {
			// 이 뒤에 리커넥팅 처리인 듯 한데.. 좀더 분석 필요.
			std::cout << "refused" << std::endl;
		}

		std::cout << "did closed" << std::endl;
	}

	void OnConnected(rtc::AsyncSocket* socket) {
		std::cout << "on connected" << std::endl;

		//if onconnect data가 있으면 보낸다.
		std::string hello = "GET / HTTP/1.0\r\nContent-Length:0\r\nConnection:Keep-Alive\r\n\r\n";
		auto sent = socket->Send(hello.c_str(), hello.size());
		if (sent <= 0) {
			std::cout << "unsent hello" << std::endl;
		}
	}

	void OnRead(rtc::AsyncSocket* socket) {
		std::cout << "on read" << std::endl;

		std::array<uint8_t, 65536> buf;
		while (true) {
			auto bytes = socket->Recv(buf.data(), buf.size(), nullptr);
			if (bytes <= 0) {
				break;
			}

			//m_reading_buf.emplace_back(buf.begin(), buf.end());
			m_reading_buf.insert(m_reading_buf.end(), buf.begin(), buf.begin() + bytes);
		}

		auto tmp = m_reading_buf;
		tmp.push_back('\0');
		std::cout << "BUF : [" << tmp.data() << "]" << std::endl;
		
		enum class Delim {
			none, r1, n1, r2, n2,
		} delimiter_state = Delim::none;
		static const struct {
			Delim cur_state;
			char cur_ch;
			Delim next_state;
			bool split;
		} s_parser[] = {
			{ Delim::none, '\r', Delim::r1, false },
			{ Delim::r1, '\n', Delim::n1, false },
			{ Delim::n1, '\r', Delim::r2, false },
			{ Delim::r2, '\n', Delim::none, true },
		};

		auto i = 0;
		while (i < m_reading_buf.size()) {
			bool matched = false;
			for (const auto & parser : s_parser) {
				if (parser.cur_state == delimiter_state && parser.cur_ch == m_reading_buf[i]) {
					delimiter_state = parser.next_state;

					if (parser.split) {
						std::string line{ m_reading_buf.begin(), m_reading_buf.begin() + i + 1 };
						line.push_back('\0');
						m_reading_buf.erase(m_reading_buf.begin(), m_reading_buf.begin() + i + 1);
						i = -1;

						std::cout << "LINE : [" << line << "]" << std::endl;
					}

					matched = true;
					break;
				}
			}

			if (!matched) {
				delimiter_state = Delim::none;
			}

			i++;
		}
	}

	void OnWrite(rtc::AsyncSocket* socket) {
		std::cout << "on write" << std::endl;
		// do nothing
	}

public:
	ControlClient() : rtc::MessageHandler(false) {

	}

	~ControlClient() {
		if (m_resolver) {
			m_resolver->Destroy(false);
			m_resolver.reset();
		}
	}

	void Run() {
		const std::string server = "127.0.0.1";
		int port = 5000;

		m_serveraddr.SetIP(server);
		m_serveraddr.SetPort(port);

		if (m_serveraddr.IsUnresolvedIP()) {
			if (!m_resolver)
				m_resolver = std::make_unique<rtc::AsyncResolver>();
			m_resolver->SignalDone.connect(this, &ControlClient::OnResolved);
			m_resolver->Start(m_serveraddr);
			std::cout << "resolving" << std::endl;
		}
		else {
			DoConnect();
		}
	}

};

int main()
{
	rtc::WinsockInitializer wsinit;
	rtc::Win32SocketServer w32_ss;
	rtc::Win32Thread w32_thread(&w32_ss);
	rtc::ThreadManager::Instance()->SetCurrentThread(&w32_thread);

	ControlClient cc;
	cc.Run();

	std::cout << "Hello World!\n";

	MSG msg;
	while (::GetMessage(&msg, nullptr, 0, 0) != 0) {
		::TranslateMessage(&msg);
		::DispatchMessage(&msg);
	}
	
}