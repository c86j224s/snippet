// NatTraversal.cpp
// c86j224s@gmail.com
//  
// UPnP-enabled NAT device ȯ�濡�� ����� �� �ִ� NAT Traversal ���.
// 
// ���� StaticPortMappingCollection ��ü�� null�� ���ϵȴٸ�, ���� ����� �õ��Ѵ�.
// -> Windows Vista �̻� �ü���� ��� ���̰�, ��ȭ�� ������ ���� �ִٸ�, 
// ��� ���α׷� ��Ͽ��� "Windows �Ǿ� �� �Ǿ� ���� �۾� �Ŀ�̼�" �� ����Ѵ�.
//
// HISTORY:
// 2012-10-23 ���� �ۼ�. NAT ��ġ�� ��ü ���� ������ ��ȸ�ϴ� ����� main �Լ� �ȿ� �ۼ�.
// 2012-10-24 �ϳ��� main �Լ��� Ŭ������ ���ۼ�.
// 2012-10-25 ���� ������ �߰�, ����, �����ϴ� ����� ����. Ȯ�� ���, �߰�/���� ���� ����. ������ �߰� �� ���Ϲ޴� OLE ��ü�� �����ϴٰ� ����.
// 2012-10-29 OLE ��ü�� ��ü ������ ���� �����ϵ��� Ŭ���� ������ ���� ��.
// 2012-10-30 �߰� �� ���Ϲ��� OLE ��ü�� �״�� �ᵵ ������ �Ұ������� Ȯ����.
//
// TODO:
// (ING) �ݷ��� �������� �� �߰�/����/������ �����Ͽ� ����Ʈ�� �����ϵ��� �۾�.
// (ING) ���� ó���� ��ȭ�ϰ�, ��ü�� �����ϰ�, Ȯ���ϰ� �����ǵ��� ����.
// �̺�Ʈ �Ŵ����� �ۼ��ؼ�, ��Ʈ ������ ������ ���� ������ ��Ƽ�޵��� ����.
// ����������, ������ ������, �ݺ��ؼ�, ���� �����忡�� ���ÿ� ������ �غ���.

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