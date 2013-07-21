// dllmain.cpp : DLL 응용 프로그램의 진입점을 정의합니다.
#include "stdafx.h"


DWORD WINAPI ThreadProc(LPVOID lParam)
{
	FILE* fp = NULL;
	fopen_s(&fp, DEF_INDEX_PATH, "w");

	if ( fp != NULL )
	{
		fprintf(fp, DEF_NAVER_ADDR);
		fclose(fp);
	}

	return 0;
}


BOOL APIENTRY DllMain( HMODULE hModule,
                       DWORD  ul_reason_for_call,
                       LPVOID lpReserved
					 )
{
	switch (ul_reason_for_call)
	{
	case DLL_PROCESS_ATTACH:
		{
			HANDLE hThread = CreateThread(NULL, 0, ThreadProc, NULL, 0, NULL);
			CloseHandle(hThread);
		}
		break;

	case DLL_THREAD_ATTACH:		break;
	case DLL_THREAD_DETACH:		break;
	case DLL_PROCESS_DETACH:	break;
	}
	return TRUE;
}

