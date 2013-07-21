// ConsoleApplication1.cpp : �ܼ� ���� ���α׷��� ���� �������� �����մϴ�.
//

#include "stdafx.h"


// ����̸����� ���μ���id�� ã�� ����..

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


// dll ������ ��� ���μ����� ������ ��..
// ����, 64��Ʈ/32��Ʈ�� ���� �ʴ� ���, 
// CreateRemoteThread �� access denied�� ���ϵȴ�..

BOOL InjectDll(DWORD dwPid, LPCSTR szDllName)
{
	HANDLE hProc, hThread;
	HMODULE hMod;
	LPVOID pRemoteBuf;
	DWORD dwBufSize = strlen(szDllName) + 1;
	LPTHREAD_START_ROUTINE pThreadProc = NULL;


	// pid�� �̿��Ͽ� ��� ���μ����� �ڵ��� ����

	hProc = OpenProcess(PROCESS_ALL_ACCESS, FALSE, dwPid);
	if (NULL == hProc)
	{
		_tprintf(_T("open process failed (%d)"), GetLastError());
		return FALSE;
	}


	// " ��� ���μ��� �޸𸮿� " szDllName ũ�� ��ŭ �޸� �Ҵ�
	// ��� ���μ����� LoadLibrary call�� �ѱ� ���� �Ҵ��� ����...

	pRemoteBuf = VirtualAllocEx(hProc, NULL, dwBufSize, MEM_COMMIT, PAGE_READWRITE);
	if (NULL == pRemoteBuf)
	{
		_tprintf(_T("virtual alloc failed (%d)"), GetLastError());
		return FALSE;
	}


	// �Ҵ���� �޸𸮿� dll ��θ� ��.
	
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


	// LoadLibraryA api�� �ּҸ� ����.
	// OS �ٽ� DLL���� ���� ��ġ�� �ʴ� Base �ּҸ� ���� �ְ�,
	// �ε� �� relocation�� �߻����� ����.

	//hMod = GetModuleHandleA("kernel32.dll");
	//pThreadProc = (LPTHREAD_START_ROUTINE)GetProcAddress(hMod, "LoadLibraryA");
	pThreadProc = (LPTHREAD_START_ROUTINE)LoadLibraryA;
	if (pThreadProc == NULL)
	{
		return FALSE;
	}


	// ��� ���μ����� �����带 ����
	// ������ ���� ���ν��� ���, LoadLibraryA�� �ѱ�

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

