// NatTraversal.cpp
// c86j224s@gmail.com
//  
// UPnP-enabled NAT device 환경에서 사용할 수 있는 NAT Traversal 방법.
// 
// 만일 StaticPortMappingCollection 객체가 null로 리턴된다면, 다음 방법을 시도한다.
// -> Windows Vista 이상 운영체제를 사용 중이고, 방화벽 설정이 켜져 있다면, 
// 허용 프로그램 목록에서 "Windows 피어 투 피어 공동 작업 파운데이션" 을 허용한다.
//
// HISTORY:
// 2012-10-23 최초 작성. NAT 장치의 전체 매핑 정보를 조회하는 기능을 main 함수 안에 작성.
// 2012-10-24 하나의 main 함수를 클래스로 재작성.
// 2012-10-25 매핑 정보를 추가, 삭제, 변경하는 기능을 구현. 확인 결과, 추가/삭제 정상 동작. 변경은 추가 시 리턴받는 OLE 객체만 가능하다고 추측.
// 2012-10-29 OLE 객체를 객체 내에서 유지 관리하도록 클래스 구조를 수정 중.
// 2012-10-30 추가 시 리턴받은 OLE 객체를 그대로 써도 변경이 불가능함을 확인함.
//
// TODO:
// (ING) 콜렉션 리프레시 시 추가/변경/삭제를 구분하여 리스트를 수정하도록 작업.
// (ING) 예외 처리를 강화하고, 객체가 안전하게, 확실하게 해제되도록 하자.
// 이벤트 매니저를 작성해서, 포트 포워딩 정보의 변경 사항을 노티받도록 하자.
// 최종적으로, 문제가 없는지, 반복해서, 여러 쓰레드에서 동시에 실행을 해보자.

#include "stdafx.h"

#include "PortMappingInfo.h"
#include "NatTraversal.h"

//==============================================================================
void programMenu(void)
{
	std::wcout << std::endl
		<< L"==[menu]================================" << std::endl
		<< L" * count" << std::endl
		<< L" * all" << std::endl
		<< L" * info [external port] [protocol]" << std::endl
		<< L" * add [external port] [protocol] [internal port] [internal client]" << std::endl
		<< L"       [enable|disable] [description]" << std::endl
		<< L" * remove [external port] [protocol]" << std::endl
		<< L" * enable [external port] [protocol] [enable|disable]" << std::endl
		<< L" * desc [exteranl port] [protocol] [description]" << std::endl
		<< L" * quit" << std::endl << std::endl
		<< L"SELECT >> ";
}

//==============================================================================
void displayError(CString msg, CError err)
{
	std::wcout << msg.GetBuffer() << std::endl;
	std::wcout << L"HR: " << err.GetResult() << std::endl;
	std::wcout << L"Src: " << err.GetString().GetBuffer() << std::endl;

	msg.ReleaseBuffer();
	err.GetString().ReleaseBuffer();
}

//==============================================================================
void displayPortMapping(CPortMappingInfo& pmInfo)
{
	std::wcout << L"enabled: " << pmInfo.GetEnabled() << std::endl
		<< L"protocol: " << pmInfo.GetProtocol().GetBuffer() << std::endl
		<< L"description: " << pmInfo.GetDescription().GetBuffer() << std::endl
		<< L"internal client: " << pmInfo.GetInternalClient().GetBuffer() << std::endl
		<< L"internal port: " << pmInfo.GetInternalPort() << std::endl
		<< L"external ip address: " << pmInfo.GetExternalIPAddress().GetBuffer() << std::endl
		<< L"external port: " << pmInfo.GetExternalPort() << std::endl
		<< std::endl;
}

//==============================================================================
int _tmain(int argc, _TCHAR* argv[])
{
	CError err;
	CNatTraversal natTraversal;
	
	err = natTraversal.Init();
	if (err.GetResult() != S_OK)
	{
		displayError(L"nat traversal initialize failed.", err);
		return -1;
	}

	err = natTraversal.Refresh();
	if (err.GetResult() != S_OK)
	{
		displayError(L"nat traversal refresh failed.", err);
		natTraversal.Fin();
		return -2;
	}

	while (true)
	{
		wchar_t menuSelect[16] = {0,};

		programMenu();
		
		std::wcin >> menuSelect;

		if (0 == ::wcscmp(menuSelect, L"count"))
		{
			if (natTraversal.GetCount() < 0)
			{
				std::wcout << L"getting port mapping count failed." << std::endl;
			}
			else
			{
				std::wcout << L"port mapping count: " << natTraversal.GetCount() << std::endl;
			}
		}
		else if (0 == ::wcscmp(menuSelect, L"all"))
		{
			for (int i = 0; i < natTraversal.GetCount(); i++)
			{
				CPortMappingInfo* pmInfo = natTraversal.GetPortMappingInfo(i);
				if (NULL != pmInfo)
				{
					displayPortMapping(*pmInfo);
				}
			}
		}
		else if (0 == ::wcscmp(menuSelect, L"info"))
		{
			long int externalPort = 0;
			wchar_t protocol[16] = {0,};

			std::wcin >> externalPort;
			std::wcin >> menuSelect;

			CPortMappingInfo* pmInfo = natTraversal.GetPortMappingInfo(externalPort, menuSelect);
			if (NULL != pmInfo)
			{
				displayPortMapping(*pmInfo);
			}
		}
		else if (0 == ::wcscmp(menuSelect, L"add"))
		{
			long int externalPort = 0;
			wchar_t protocol[16] = {0,};
			long int internalPort = 0;
			wchar_t internalClient[32] = {0,};
			wchar_t enableStr[16] = {0,};
			wchar_t description[32] = {0,};

			BOOL enable = FALSE;

			std::wcin >> externalPort;
			std::wcin >> protocol;
			std::wcin >> internalPort;
			std::wcin >> internalClient;
			std::wcin >> enableStr;
			std::wcin >>description;

			if (0 == ::wcscmp(enableStr, L"enable"))
			{
				enable = TRUE;
			}

			CPortMappingInfo* pmInfo = natTraversal.AddPortMapping(externalPort, protocol, internalPort, internalClient, enable, description);
			if (NULL != pmInfo)
			{
				std::wcout << L"succeed" << std::endl;
			}
		}
		else if (0 == ::wcscmp(menuSelect, L"remove"))
		{
			long int externalPort = 0;
			wchar_t protocol[16] = {0,};

			std::wcin >> externalPort;
			std::wcin >> protocol;

			natTraversal.RemovePortMapping(externalPort, protocol);
		}
		else if (0 == ::wcscmp(menuSelect, L"enable"))
		{
			long int externalPort = 0;
			wchar_t protocol[16] = {0,};
			wchar_t enableStr[16] = {0,};

			BOOL enable = FALSE;

			std::wcin >> externalPort;
			std::wcin >> protocol;
			std::wcin >> enableStr;

			if (0 == ::wcscmp(enableStr, L"enable"))
			{
				enable = TRUE;
			}

			CPortMappingInfo* pmInfo = natTraversal.GetPortMappingInfo(externalPort, protocol);
			if (NULL != pmInfo)
			{
				CError err = pmInfo->SetEnabled(enable);
				if (err.GetResult() != S_OK)
				{
					displayError(L"editing port mapping enable failed.", err);
				}
				else
				{
					std::wcout << "succeed." << std::endl;
				}
			}
		}
		else if (0 == ::wcscmp(menuSelect, L"desc"))
		{
			long int externalPort = 0;
			wchar_t protocol[16] = {0,};
			wchar_t description[32] = {0,};

			std::wcin >> externalPort;
			std::wcin >> protocol;
			std::wcin >> description;

			CPortMappingInfo* pmInfo = natTraversal.GetPortMappingInfo(externalPort, protocol);
			if (NULL != pmInfo)
			{
				CError err = pmInfo->SetDescription(description);
				if (err.GetResult() != S_OK)
				{
					displayError(L"editing port mapping description failed.", err);
				}
				else
				{
					std::wcout << "succeed." << std::endl;
				}
			}
		}
		else if (0 == ::wcscmp(menuSelect, L"quit"))
		{
			break;
		}
	}

	natTraversal.Fin();
	
	return 0;
}


//==============================================================================