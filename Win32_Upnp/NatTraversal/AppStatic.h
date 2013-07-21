#pragma once

//==============================================================================
class CWinsock2
{
public:
	CWinsock2() { WSADATA wd; WSAStartup(MAKEWORD(2,2), &wd); }
	~CWinsock2() { WSACleanup(); }
};

//==============================================================================
class CComPlus
{
public:
	CComPlus() { CoInitialize(NULL); }
	~CComPlus() { CoUninitialize(); }
};

//==============================================================================
extern CWinsock2 g_winsock2;
extern CComPlus g_complus;

//==============================================================================