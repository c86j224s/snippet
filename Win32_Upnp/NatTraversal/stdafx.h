// stdafx.h : ���� ��������� ���� ��������� �ʴ�
// ǥ�� �ý��� ���� ���� �� ������Ʈ ���� ���� ������
// ��� �ִ� ���� �����Դϴ�.
//

#pragma once

#include "targetver.h"

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <tchar.h>


#define _ATL_CSTRING_EXPLICIT_CONSTRUCTORS      // �Ϻ� CString �����ڴ� ��������� ����˴ϴ�.

#include <atlbase.h>
#include <atlstr.h>



// TODO: ���α׷��� �ʿ��� �߰� ����� ���⿡�� �����մϴ�.

#include <iostream>
#include <vector>

#include <winsock2.h>
#pragma comment(lib, "ws2_32.lib")

#include <windows.h>

//#include <iphlapi.h>
//#pragma comment(lib, "iphlapi.lib")

#include <natupnp.h>


#include <comdef.h>
#include <comutil.h>
#if defined(_WCHAR_T_DEFINED) || defined(_WCHAR_DEFINED) || defined(_NATIVE_WCHAR_T_DEFINED)
#pragma comment (lib, "comsuppw.lib")
#else
#pragma comment (lib, "comsupp.lib")
#endif

#include "AppStatic.h"
#include "Error.h"