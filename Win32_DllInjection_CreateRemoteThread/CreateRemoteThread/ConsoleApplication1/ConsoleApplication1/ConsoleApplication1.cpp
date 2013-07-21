// ConsoleApplication1.cpp : 콘솔 응용 프로그램에 대한 진입점을 정의합니다.
//

#include "stdafx.h"


// 모듈이름으로 프로세스id를 찾아 리턴..

DWORD FindPid(LPCTSTR szProcName)
{
	DWORD dwPid = -1;
	HANDLE hSnapShot = INVALID_HANDLE_VALUE;
	PROCESSENTRY32W pe = {0,};

	// get the snapshot of the system
	pe.dwSize = sizeof(PROCESSENTRY32W);
	hSnapShot = CreateToolhelp32Snapshot(TH32CS_SNAPALL, NULL);

	// find proc
	Process32First(hSnapShot, &pe);
	do
	{
		if (!_tcsicmp(szProcName, pe.szExeFile))
		{
			dwPid = pe.th32ProcessID;
			break;
		}
	}
	while(Process32NextW(hSnapShot, &pe));

	CloseHandle(hSnapShot);

	return dwPid;
}


// dll 파일을 대상 프로세스에 인젝션 함..
// 만일, 64비트/32비트가 맞지 않는 경우, 
// CreateRemoteThread 시 access denied가 리턴된다..

BOOL InjectDll(DWORD dwPid, LPCSTR szDllName)
{
	HANDLE hProc, hThread;
	HMODULE hMod;
	LPVOID pRemoteBuf;
	DWORD dwBufSize = strlen(szDllName) + 1;
	LPTHREAD_START_ROUTINE pThreadProc = NULL;


	// pid를 이용하여 대상 프로세스의 핸들을 구함

	hProc = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwPid);
	if (NULL == hProc)
	{
		_tprintf(_T("open process failed (%d)"), GetLastError());
		return FALSE;
	}


	// " 대상 프로세스 메모리에 " szDllName 크기 만큼 메모리 할당
	// 대상 프로세스의 LoadLibrary call에 넘길 인자 할당을 위함...

	pRemoteBuf = VirtualAllocEx(hProc, NULL, dwBufSize, MEM_COMMIT, PAGE_READWRITE);
	if (NULL == pRemoteBuf)
	{
		_tprintf(_T("virtual alloc failed (%d)"), GetLastError());
		return FALSE;
	}


	// 할당받은 메모리에 dll 경로를 씀.
	
	SIZE_T dwWritten = 0;
	if (FALSE == WriteProcessMemory(hProc, pRemoteBuf, (LPVOID)szDllName, dwBufSize, &dwWritten))
	{
		_tprintf(_T("write process failed (%d)"), GetLastError());
		return FALSE;
	}

	if (dwWritten != dwBufSize)
	{
		_tprintf(_T("write process failed (%d)"), GetLastError());
		return FALSE;
	}


	// LoadLibraryA api의 주소를 구함.
	// OS 핵심 DLL들은 서로 겹치지 않는 Base 주소를 갖고 있고,
	// 로딩 시 relocation이 발생하지 않음.

	//hMod = GetModuleHandleA("kernel32.dll");
	//pThreadProc = (LPTHREAD_START_ROUTINE)GetProcAddress(hMod, "LoadLibraryA");
	pThreadProc = (LPTHREAD_START_ROUTINE)LoadLibraryA;
	if (pThreadProc == NULL)
	{
		return FALSE;
	}


	// 대상 프로세스에 스레드를 실행
	// 스레드 메인 프로시저 대신, LoadLibraryA를 넘김

	hThread = CreateRemoteThread(hProc, NULL, 0, pThreadProc, pRemoteBuf, 0, NULL);
	if (hThread == NULL)
	{
		_tprintf(_T("create remote thread failed (%d)"), GetLastError());
		return FALSE;
	}

	WaitForSingleObject(hThread, INFINITE);

	CloseHandle(hThread);
	CloseHandle(hProc);

	return TRUE;

}

int _tmain(int argc, _TCHAR* argv[])
{
	DWORD dwPid = -1;

	// find proc

	dwPid = FindPid(DEF_PROG_NAME);
	if (dwPid == -1)
	{
		_tprintf(_T("There is no <%s> process\n"), DEF_PROG_NAME);
		return 1;
	}

	// inject dll

	if (InjectDll(dwPid, DEF_DLL_PATH) != TRUE)
	{
		return 1;
	}

	return 0;
}

