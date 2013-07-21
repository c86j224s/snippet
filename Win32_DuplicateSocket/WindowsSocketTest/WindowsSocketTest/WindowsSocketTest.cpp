// WindowsSocketTest.cpp : 콘솔 응용 프로그램에 대한 진입점을 정의합니다.
//

#include "stdafx.h"

//////////////////////////////////////////////////////////////////////////

/*
 * Winsock2 API 중 WSADuplicateSocket을 테스트해보기 위해 작성했습니다..
 * 싱글스레드에, 예외 처리는 거의 신경 쓰지 않았고, 락도 하나도 걸려 있지 않고,...
 * 온전히 WSADuplicateSocket 테스트를 위해서 급히 작성한 코드로서..  
 * 결국 못 써먹을 코드이므로 오로지 참고 용도로만 적절합니다.
 */

//////////////////////////////////////////////////////////////////////////

#define BYTELEN_COMMAND 4

#define COMMAND_FORWARD "FWRD"	// payloads : processId (int)

#define COMMAND_CONNECT "CONN"	// no payloads

//////////////////////////////////////////////////////////////////////////
// 윈속 초기화와 사용 해제를 위한 전역 객체입니다.
struct socket_starter
{
	socket_starter() { WSADATA wd; WSAStartup( MAKEWORD(2,2), &wd ); }
	~socket_starter() { WSACleanup(); }
};
socket_starter g_socket_starter;

//////////////////////////////////////////////////////////////////////////
// 연결 들어온 에코 서버 별로 소켓을 담아두는 곳입니다.
map<int, SOCKET> g_procSockMap;

//////////////////////////////////////////////////////////////////////////
// 오로지 accept를 받기만 하고, 받은 session을 다른 echo svr로 전달만 해주는 서버의 main routine 입니다.
// 싱글 스레드이기 때문에 한번에 하나의 연결만 처리합니다.

int run_acceptor()
{
	// 리스닝 소켓을 만듭니다.

	struct sockaddr_in sa;

	sa.sin_family = AF_INET;
	sa.sin_addr.S_un.S_addr = INADDR_ANY;
	sa.sin_port = htons(3000);				// acceptor 서버 포트

	SOCKET ls = socket( AF_INET, SOCK_STREAM, IPPROTO_TCP );
	if ( ls == INVALID_SOCKET )
	{
		throw _T("socket error!\n");
	}

	if ( bind( ls, (sockaddr*)&sa, sizeof(sa) ) == SOCKET_ERROR )
	{
		throw _T("bind error\n");
	}

	if ( listen( ls, 5 ) < 0 )
	{
		throw _T("listen error!\n");
	};

	_tprintf( _T("listening.. \n") );

	// accept 루프를 돕니다.

	while (true)
	{
		int addrLen = sizeof(sa);
		SOCKET as = accept( ls, (sockaddr*)&sa, &addrLen );
		if ( as == SOCKET_ERROR )
		{
			throw _T("accept error!");
		}

		_tprintf( _T("new session established.\n") );

		char buff[BYTELEN_COMMAND+1] = {0,};
		
		// 커맨드를 읽어들입니다. 
		// 모든 세션의 첫 4바이트는 커맨드입니다. 첫 4바이트로 모르는 커맨드가 들어오면 끊어버립니다.
		if ( recv( as, buff, BYTELEN_COMMAND, 0 ) != BYTELEN_COMMAND )
		{
			_tprintf( _T("invalid session. force to close socket.\n") );
			closesocket( as );

			continue;
		}

		// 패킷을 파싱합니다.

		if ( 0 == strcmp( buff, COMMAND_FORWARD ) )
		{
			// FORWARD 명령을 받으면, acceptor는 그 세션을 기억해두었다가, 나중에 들어오는 echosvr 세션의 소켓을 복사해서 넘겨줍니다.

			// processId
			int processId = 0;
			if ( recv( as, (char*)&processId, sizeof(processId), 0 ) != sizeof(processId) )
			{
				_tprintf( _T("FORWARD packet containes invalid payloads. force to close socket.\n") );
				closesocket( as );

				continue;
			}

			_tprintf( _T("echosvr %d subscribes client sessions.\n"), processId );
			g_procSockMap.insert( map<int,SOCKET>::value_type(processId, as) );

			continue;
		}
		else if ( 0 == strcmp( buff, COMMAND_CONNECT ) )
		{
			// CONNECT 명령을 받으면, acceptor는 기억해두었던 echosvr 세션 중 하나로 이 세션을 복사해서 넘겨줍니다.

			WSAPROTOCOL_INFOW protoInfo = {0,};
			map<int,SOCKET>::iterator it = g_procSockMap.begin();

			// 소켓을 복사합니다.

			if ( WSADuplicateSocket( as, it->first, &protoInfo ) != 0 )
			{
				_tprintf( _T("socket duplicate failed.\n") );
				continue;
			}

			_tprintf( _T("socket duplicated.\n") );

			// 소켓을 echosvr에게 넘겨줍니다.

			if ( send( it->second, (char*)&protoInfo, sizeof(WSAPROTOCOL_INFOW), 0 ) == sizeof(WSAPROTOCOL_INFOW) )
			{
				// 싱글 스레드이므로.. 당장 안쓰는 소켓은 남겨둘 이유가 없습니다.
				// 복사해서 넘겨준 세션도 끊고... echosvr 세션도 끊고...
				// echosvr가 또다른 세션을 넘겨받고 싶으면, acceptor에 다시금 연결해 들어와야 합니다.

				_tprintf( _T("duplicated socket shared.\n") );
				closesocket(as);
				closesocket(it->second);
				g_procSockMap.erase( it );
			}
				
			continue;
		}
		
	}

	return 0;
}

//////////////////////////////////////////////////////////////////////////
// acceptor에서 클라이언트 세션을 넘겨받아, echo 서비스를 하는 서버입니다.
// 이 서버 역시 싱글스레드입니다.
// acceptor로부터 한번에 하나의 클라이언트 세션만 받아서 처리하고, 
// 처리가 다 끝나면 다시 acceptor에 연결하여 다음 클라이언트 세션을 받아옵니다.

int run_echosvr()
{
	while ( true )
	{
		struct sockaddr_in sa;

		sa.sin_family = AF_INET;
		sa.sin_addr.S_un.S_addr = inet_addr("127.0.0.1");
		sa.sin_port = htons(3000);

		SOCKET cs = socket( AF_INET, SOCK_STREAM, IPPROTO_TCP );
		if ( cs == INVALID_SOCKET )
		{
			throw _T("socket error");
		}

		if ( connect( cs, (const sockaddr*)&sa, sizeof(sa) ) == SOCKET_ERROR )
		{
			throw _T("connect error");
		}

		// acceptor를 위한 패킷을 만들어 보냅니다.

		char buff[8] = COMMAND_FORWARD;
		*(int*)(&buff[4]) = GetCurrentProcessId();

		if ( send( cs, buff, 8, 0 ) == 8 )
		{
			WSAPROTOCOL_INFOW protoInfo = {0,};

			if ( recv( cs, (char*)&protoInfo, sizeof(WSAPROTOCOL_INFOW), 0 ) == sizeof(WSAPROTOCOL_INFOW) )
			{
				SOCKET ds = WSASocket( AF_INET, SOCK_STREAM, IPPROTO_TCP, &protoInfo, 0, 0 );
				if ( ds == SOCKET_ERROR )
				{
					_tprintf( _T("failed to share duplicated socket.") );
					continue;
				}

				char ch;

				// 그만하는 방법은, 클라이언트 쪽에서 세션을 끊는 것 뿐입니다.
				// ...그런데, 이름은 echosvr인데 하는 짓은 echo가 아니다......
				// (테스트하는덴 상관 없음)

				while ( recv( ds, &ch, 1, 0 ) == 1 ) 
				{
					putc( ch, stdout );
				}
			}

		}

		closesocket( cs );
	}
	
	return 0;
}

//////////////////////////////////////////////////////////////////////////
int run_client()
{
	struct sockaddr_in sa;

	sa.sin_family = AF_INET;
	sa.sin_addr.S_un.S_addr = inet_addr("127.0.0.1");
	sa.sin_port = htons(3000);

	SOCKET cs = socket( AF_INET, SOCK_STREAM, IPPROTO_TCP );
	if ( cs == INVALID_SOCKET )
	{
		throw _T("socket error");
	}

	if ( connect( cs, (const sockaddr*)&sa, sizeof(sa) ) == SOCKET_ERROR )
	{
		throw _T("connect error");
	}

	char buff[BYTELEN_COMMAND+1] = COMMAND_CONNECT;
	if ( send( cs, (char*)buff, BYTELEN_COMMAND, 0 ) == BYTELEN_COMMAND )
	{
		char ch;
		while( ch = getc(stdin) )
		{
			send( cs, &ch, 1, 0 );
		}
	}

	closesocket( cs );

	return 0;
}


//////////////////////////////////////////////////////////////////////////
int _tmain(int argc, _TCHAR* argv[])
{
	if (argc < 2)
	{
		_tprintf( _T("usage : %s [/acceptor|/echosvr|/client]\n"), argv[0] );
		return 0;
	}

	if ( 0 == _tcscmp( argv[1], _T("/acceptor") ) )
	{
		try { run_acceptor(); } 
		catch (LPTSTR tStr) { _tprintf( tStr ); } 
		catch (...) { _tprintf( _T("unexpected error") ); }
	}
	else if ( 0 == _tcscmp( argv[1], _T("/echosvr") ) )
	{
		try { run_echosvr(); }
		catch (LPTSTR tStr) { _tprintf( tStr ); }
		catch (...) { _tprintf( _T("unexpected error") ); }
	}
	else if ( 0 == _tcscmp( argv[1], _T("/client") ) )
	{
		try { run_client(); }
		catch (LPTSTR tStr) { _tprintf( tStr ); }
		catch (...) { _tprintf( _T("unexpected error") ); }
	}
	else
	{
		_tprintf( _T("invalid app type") );
		return 0;
	}

	return 0;
}

