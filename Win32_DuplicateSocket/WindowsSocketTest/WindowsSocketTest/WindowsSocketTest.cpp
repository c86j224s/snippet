// WindowsSocketTest.cpp : �ܼ� ���� ���α׷��� ���� �������� �����մϴ�.
//

#include "stdafx.h"

//////////////////////////////////////////////////////////////////////////

/*
 * Winsock2 API �� WSADuplicateSocket�� �׽�Ʈ�غ��� ���� �ۼ��߽��ϴ�..
 * �̱۽����忡, ���� ó���� ���� �Ű� ���� �ʾҰ�, ���� �ϳ��� �ɷ� ���� �ʰ�,...
 * ������ WSADuplicateSocket �׽�Ʈ�� ���ؼ� ���� �ۼ��� �ڵ�μ�..  
 * �ᱹ �� ����� �ڵ��̹Ƿ� ������ ���� �뵵�θ� �����մϴ�.
 */

//////////////////////////////////////////////////////////////////////////

#define BYTELEN_COMMAND 4

#define COMMAND_FORWARD "FWRD"	// payloads : processId (int)

#define COMMAND_CONNECT "CONN"	// no payloads

//////////////////////////////////////////////////////////////////////////
// ���� �ʱ�ȭ�� ��� ������ ���� ���� ��ü�Դϴ�.
struct socket_starter
{
	socket_starter() { WSADATA wd; WSAStartup( MAKEWORD(2,2), &wd ); }
	~socket_starter() { WSACleanup(); }
};
socket_starter g_socket_starter;

//////////////////////////////////////////////////////////////////////////
// ���� ���� ���� ���� ���� ������ ��Ƶδ� ���Դϴ�.
map<int, SOCKET> g_procSockMap;

//////////////////////////////////////////////////////////////////////////
// ������ accept�� �ޱ⸸ �ϰ�, ���� session�� �ٸ� echo svr�� ���޸� ���ִ� ������ main routine �Դϴ�.
// �̱� �������̱� ������ �ѹ��� �ϳ��� ���Ḹ ó���մϴ�.

int run_acceptor()
{
	// ������ ������ ����ϴ�.

	struct sockaddr_in sa;

	sa.sin_family = AF_INET;
	sa.sin_addr.S_un.S_addr = INADDR_ANY;
	sa.sin_port = htons(3000);				// acceptor ���� ��Ʈ

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

	// accept ������ ���ϴ�.

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
		
		// Ŀ�ǵ带 �о���Դϴ�. 
		// ��� ������ ù 4����Ʈ�� Ŀ�ǵ��Դϴ�. ù 4����Ʈ�� �𸣴� Ŀ�ǵ尡 ������ ��������ϴ�.
		if ( recv( as, buff, BYTELEN_COMMAND, 0 ) != BYTELEN_COMMAND )
		{
			_tprintf( _T("invalid session. force to close socket.\n") );
			closesocket( as );

			continue;
		}

		// ��Ŷ�� �Ľ��մϴ�.

		if ( 0 == strcmp( buff, COMMAND_FORWARD ) )
		{
			// FORWARD ����� ������, acceptor�� �� ������ ����صξ��ٰ�, ���߿� ������ echosvr ������ ������ �����ؼ� �Ѱ��ݴϴ�.

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
			// CONNECT ����� ������, acceptor�� ����صξ��� echosvr ���� �� �ϳ��� �� ������ �����ؼ� �Ѱ��ݴϴ�.

			WSAPROTOCOL_INFOW protoInfo = {0,};
			map<int,SOCKET>::iterator it = g_procSockMap.begin();

			// ������ �����մϴ�.

			if ( WSADuplicateSocket( as, it->first, &protoInfo ) != 0 )
			{
				_tprintf( _T("socket duplicate failed.\n") );
				continue;
			}

			_tprintf( _T("socket duplicated.\n") );

			// ������ echosvr���� �Ѱ��ݴϴ�.

			if ( send( it->second, (char*)&protoInfo, sizeof(WSAPROTOCOL_INFOW), 0 ) == sizeof(WSAPROTOCOL_INFOW) )
			{
				// �̱� �������̹Ƿ�.. ���� �Ⱦ��� ������ ���ܵ� ������ �����ϴ�.
				// �����ؼ� �Ѱ��� ���ǵ� ����... echosvr ���ǵ� ����...
				// echosvr�� �Ǵٸ� ������ �Ѱܹް� ������, acceptor�� �ٽñ� ������ ���;� �մϴ�.

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
// acceptor���� Ŭ���̾�Ʈ ������ �Ѱܹ޾�, echo ���񽺸� �ϴ� �����Դϴ�.
// �� ���� ���� �̱۽������Դϴ�.
// acceptor�κ��� �ѹ��� �ϳ��� Ŭ���̾�Ʈ ���Ǹ� �޾Ƽ� ó���ϰ�, 
// ó���� �� ������ �ٽ� acceptor�� �����Ͽ� ���� Ŭ���̾�Ʈ ������ �޾ƿɴϴ�.

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

		// acceptor�� ���� ��Ŷ�� ����� �����ϴ�.

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

				// �׸��ϴ� �����, Ŭ���̾�Ʈ �ʿ��� ������ ���� �� ���Դϴ�.
				// ...�׷���, �̸��� echosvr�ε� �ϴ� ���� echo�� �ƴϴ�......
				// (�׽�Ʈ�ϴµ� ��� ����)

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

