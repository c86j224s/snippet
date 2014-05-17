// FiberSample.cpp : 
//
// ���̹��� �̿��� �ϳ��� �Լ� �ȿ��� ���� ó���� ���������� �ϴ� ���� �õ��Ѵ�.
// @todo �ڵ� ���� �������� �������� manager Ŭ������ ���� ��. (�׸��� ���� ������ �� �� �ֵ��� �������̽��� �ٽ� ������ ��.)
//
// ���̹� ������ ���� Ȯ���ؼ� ������ ���� �͵��� �Ѵ�.
// @todo select ������ �ۼ��ؼ� ��ø�� ��û ó���� �������� Ȯ���� ��.
// @todo ���� �׽�Ʈ (�󸶳� ���� ���̹��� ���� �� �ִ°�)
// @todo ���� �׽�Ʈ (���̹��� ���� ���� ���� ���ؼ� TPS ���̰� �󸶳� ���°�?)
// @todo iocp�� �̿��Ѵ�.
// @todo ���̹��� ������ �����尡 �ƴ� �ٸ� �����忡���� �ش� ���̹��� ����� �� �ִ��� Ȯ���Ѵ�.

#include "stdafx.h"

//------------------------------------------------------------

struct wsmgr {
	wsmgr() { WSADATA w; WSAStartup(MAKEWORD(2, 2), &w); }
	~wsmgr() { WSACleanup();  }
} g_wsmgr;

//------------------------------------------------------------

class FiberContext;

bool SaveContext(int tid, FiberContext* ctx);
bool EraseContext(int tid);

FiberContext* GetCurrentContext();

//------------------------------------------------------------

// ���̹� ���ؽ�Ʈ Ŭ����
class FiberContext {
public:
	LPVOID fiber_;
	FiberContext* prevFiber_;
	SOCKET socket_;
	bool running_;

	FiberContext(
		FiberContext* prevFiber = NULL, 
		LPVOID precreatedFiber = NULL
	) : fiber_(precreatedFiber), 
		prevFiber_(prevFiber) {

	}

	bool Init(SOCKET socket = INVALID_SOCKET) {
		fiber_ = CreateFiber(0, fiberCallback, this);
		socket_ = socket;
		return (fiber_ != NULL);
	}

	void SwitchTo() {
		SwitchToFiber(fiber_);
	}

	void SetSocket(SOCKET socket) {
		socket_ = socket;
	}

	virtual bool DoFiber() {
		static int subjectId = 1; // use subject header as transaction id
		char msg[128];
		
		subjectId++;
		sprintf_s(msg, _countof(msg), "GET /A.txt HTTP/1.1\r\nSubject: %d\r\nConnection: keep-alive\r\n\r\n", subjectId);
		SaveContext(subjectId, this);
		running_ = true;
		int ret = send(socket_, msg, strlen(msg), 0);
		if (ret == SOCKET_ERROR) {
			EraseContext(subjectId);
			std::cerr << "send err : " << WSAGetLastError() << std::endl;
			throw;
		}

		prevFiber_->SwitchTo();

		subjectId++;
		sprintf_s(msg, _countof(msg), "GET /B.txt HTTP/1.1\r\nSubject: %d\r\nConnection: keep-alive\r\n\r\n", subjectId);
		SaveContext(subjectId, this);
		running_ = true;
		ret = send(socket_, msg, strlen(msg), 0);
		if (ret == SOCKET_ERROR) {
			EraseContext(subjectId);
			throw;
		}

		prevFiber_->SwitchTo();

		running_ = false;
		return true;
	}
	
	static VOID __stdcall fiberCallback(LPVOID param) {
		FiberContext* fiber = (FiberContext*)param;

		while (GetCurrentContext()) {
			GetCurrentContext()->DoFiber();

			if (GetCurrentContext()->prevFiber_) {
				GetCurrentContext()->prevFiber_->SwitchTo();
			}
		}
	}
};

//------------------------------------------------------------

// ���̹� Ǯ
std::stack<FiberContext*> g_fiberPool;

//------------------------------------------------------------

// tid - ���̹� ��
std::map<unsigned int, FiberContext*> g_savedFiberMap;

bool SaveContext(int tid, FiberContext* ctx) {
	g_savedFiberMap.insert(std::make_pair(tid, ctx));
	return true;
}

bool EraseContext(int tid) {
	g_savedFiberMap.erase(tid);
	return true;
}
	
//------------------------------------------------------------

// ���� ���ؽ�Ʈ ��ü (���� ���� ���� ���̹��� ����..)
FiberContext* g_currentFiberContext;

FiberContext* GetCurrentContext() {
	return g_currentFiberContext;
}

//------------------------------------------------------------

char g_bigBigBuffer[1 * 1024 * 1024];

int _tmain(int argc, _TCHAR* argv[])
{
	int ret = 0;

	// Ŀ��Ʈ ���� ����
	SOCKET cliConn = WSASocket(AF_INET, SOCK_STREAM, IPPROTO_TCP, NULL, 0, 0);
	if (cliConn == INVALID_SOCKET) {
		return -1;
	}

	// Ŀ��Ʈ
	sockaddr_in addr;
	addr.sin_family = AF_INET;
	addr.sin_addr.S_un.S_addr = inet_addr("127.0.0.1");
	addr.sin_port = htons(8080);

	ret = WSAConnect(cliConn, (const sockaddr*)&addr, sizeof(addr), NULL, NULL, NULL, NULL);
	if (ret == SOCKET_ERROR) {
		return -2;
	}

	char tempMsg[128];
	sprintf_s(tempMsg, _countof(tempMsg), "GET /0.txt HTTP/1.1\r\nSubject: 0\r\nConnection: keep-alive\r\n\r\n");
	ret = send(cliConn, tempMsg, strlen(tempMsg), 0);

	LPVOID mainFiber = ConvertThreadToFiber(NULL);
	FiberContext mainContext(NULL, mainFiber);

	// ���� �̺�Ʈ ����Ʈ
	fd_set readfds, exceptfds;

	while (true) {
		FD_ZERO(&readfds);
		FD_ZERO(&exceptfds);
		FD_SET(cliConn, &readfds);
		FD_SET(cliConn, &exceptfds);
		
		ret = select(cliConn+1, &readfds, NULL, &exceptfds, NULL);
		if (ret == SOCKET_ERROR) {
			std::cerr << "select err : " << WSAGetLastError() << std::endl;
			return -1;
		}
		else if (ret <= 0) {
			// no event
		}
		else {
			if (FD_ISSET(cliConn, &exceptfds)) {
				FD_CLR(cliConn, &exceptfds);

				std::cerr << "socket exception" << std::endl;
				closesocket(cliConn);
				break;
			}

			if (FD_ISSET(cliConn, &readfds)) {
				recv(cliConn, g_bigBigBuffer, sizeof(g_bigBigBuffer), 0);

				std::cout << g_bigBigBuffer << std::endl;

				char* subjectIdStr = strstr(g_bigBigBuffer, "Subject:");
				if (subjectIdStr) {
					subjectIdStr += strlen("Subject:");
					int subjectId = atoi(subjectIdStr);

					std::map<unsigned int, FiberContext*>::iterator it = g_savedFiberMap.find(subjectId);
					if (g_savedFiberMap.end() != it) {
						g_currentFiberContext = it->second;
						g_savedFiberMap.erase(subjectId);
					}
				}

				if (!g_currentFiberContext) {
					if (g_fiberPool.empty()) {
						g_currentFiberContext = new FiberContext(&mainContext);
						g_currentFiberContext->Init(cliConn);
					}
					else {
						g_currentFiberContext = g_fiberPool.top();
					}
				}

				g_currentFiberContext->SwitchTo();

				if (!g_currentFiberContext->running_) {
					g_fiberPool.push(g_currentFiberContext);
				}

				FD_CLR(cliConn, &readfds);
			}
		}
		g_currentFiberContext = NULL;
		ZeroMemory(g_bigBigBuffer, sizeof(g_bigBigBuffer));
	}

	return 0;
}

