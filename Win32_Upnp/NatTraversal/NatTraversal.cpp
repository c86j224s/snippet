#include "stdafx.h"

#include "PortMappingInfo.h"
#include "NatTraversal.h"

//==============================================================================
CNatTraversal::CNatTraversal()
	: m_upnpNat(NULL)
	, m_iPortMappingCollection(NULL)
{
	// do nothing here.
}

//==============================================================================
CError CNatTraversal::Init()
{
	HRESULT hr;

	hr = CoCreateInstance(CLSID_UPnPNAT, NULL, CLSCTX_ALL, IID_IUPnPNAT, (void**)&m_upnpNat);
	if (FAILED(hr))
	{
		return CError(hr, __FILE__, __LINE__);
	}
	else if (m_upnpNat == NULL)
	{
		return CError(S_FALSE, __FILE__, __LINE__);
	}

	hr = m_upnpNat->get_StaticPortMappingCollection(&m_iPortMappingCollection);
	if (FAILED(hr))
	{
		return CError(hr, __FILE__, __LINE__);
	}
	else if (m_iPortMappingCollection == NULL)
	{
		return CError(S_FALSE, __FILE__, __LINE__);
	}

	return CError(hr, __FILE__, __LINE__);
}

//==============================================================================
void CNatTraversal::Fin()
{
	Clear();

	if (NULL != m_iPortMappingCollection)
	{
		m_iPortMappingCollection->Release();
		m_iPortMappingCollection = NULL;
	}

	if (NULL != m_upnpNat)
	{
		m_upnpNat->Release();
		m_upnpNat = NULL;
	}
}

//==============================================================================
void CNatTraversal::Clear()
{
	std::vector<CPortMappingInfo*>::iterator it = m_portMappingList.begin();
	while (it != m_portMappingList.end())
	{
		CPortMappingInfo* pmInfo = *it;
		pmInfo->Clear();

		it = m_portMappingList.erase(it);
	}
}

//==============================================================================
CError CNatTraversal::Refresh()
{
	// [TODO] 모든 요소를 다시 만드는 것이 아니라, 
	//     그대로 둔 상태에서 새로 만들어진 것, 지워진 것, 바뀐 것 따로 처리해야 함..

	Clear();

	HRESULT hr = S_OK;
	const char* file = __FILE__;
	int line = __LINE__;

	IUnknown* unknown = NULL;
	hr = m_iPortMappingCollection->get__NewEnum(&unknown);
	if (FAILED(hr)) { file = __FILE__; line = __LINE__; }
	else if (unknown == NULL) { hr = S_FALSE; file = __FILE__; line = __LINE__; }
	else
	{
		IEnumVARIANT* enumVariant;
		hr = unknown->QueryInterface(IID_IEnumVARIANT, (void**)&enumVariant );
		if (FAILED(hr)) { file = __FILE__; line = __LINE__; }
		else if (enumVariant == NULL) { hr = S_FALSE; file = __FILE__; line = __LINE__; }
		else
		{
			VARIANT variant;
			VariantInit(&variant);

			while (1)
			{
				hr = enumVariant->Next(1, &variant, NULL);
				if (FAILED(hr) && variant.vt != VT_EMPTY) { file = __FILE__; line = __LINE__; break; }
				else if (variant.vt == VT_EMPTY) { hr = S_OK; break; }	// end of enum
				else
				{
					IStaticPortMapping* tempIStaticPortMapping = NULL;
					IUnknown* variantUnknown;

					CPortMappingInfo* pmInfo = new CPortMappingInfo;
					if (pmInfo == NULL) { hr = S_FALSE; file = __FILE__; line = __LINE__; break; }

					variantUnknown = V_UNKNOWN(&variant);
					hr = variantUnknown->QueryInterface(IID_IStaticPortMapping, (void**)&tempIStaticPortMapping);
					if (FAILED(hr) || tempIStaticPortMapping == NULL) { file = __FILE__; line = __LINE__; }
					else if (tempIStaticPortMapping == NULL) { hr = S_FALSE; file = __FILE__; line = __LINE__; }
					else
					{
						pmInfo->SetIStaticPortMapping(tempIStaticPortMapping);

						CError err = pmInfo->Refresh();
						if (FAILED(err.GetResult())) { hr = err.GetResult(); file = __FILE__; line = __LINE__; break; }

						VariantClear(&variant);

						m_portMappingList.push_back(pmInfo);
					}
				}
			}

			enumVariant->Release();
			enumVariant = NULL;
		}

		unknown->Release();
		unknown = NULL;
	}

	return CError(hr, file, line);
}

//==============================================================================
int CNatTraversal::GetCount()
{
	return m_portMappingList.size();
}

//==============================================================================
CPortMappingInfo* CNatTraversal::GetPortMappingInfo(int idx)
{
	if (idx < 0 || idx >= m_portMappingList.size()) { return NULL; }

	return m_portMappingList[idx];
}

//==============================================================================
CPortMappingInfo* CNatTraversal::GetPortMappingInfo(
	long int externalPort, 
	CString protocol)
{
	std::vector<CPortMappingInfo*>::iterator it = m_portMappingList.begin();
	for (; it != m_portMappingList.end(); it++)
	{
		CPortMappingInfo* pmInfo = *it;
		
		if (pmInfo->GetExternalPort() == externalPort
			&& 0 == pmInfo->GetProtocol().Compare(protocol))
		{
			return pmInfo;
		}
	}

	return NULL;
}

//==============================================================================
CPortMappingInfo* CNatTraversal::AddPortMapping(
	long int externalPort, 
	CString protocol,
	long int internalPort,
	CString internalClient,
	BOOL enabled,
	CString description)
{
	HRESULT hr = S_OK;

	CPortMappingInfo* pmInfo = new CPortMappingInfo;
	if (pmInfo == NULL)
	{
		return NULL;
	}

	IStaticPortMapping* tempPortMapping = NULL;

	hr = m_iPortMappingCollection->Add(
		externalPort,
		protocol.GetBuffer(),
		internalPort,
		internalClient.GetBuffer(),
		(enabled ? VARIANT_TRUE:VARIANT_FALSE),
		description.GetBuffer(),
		&tempPortMapping);
	if (FAILED(hr) || NULL == tempPortMapping)
	{
		delete pmInfo;
		return NULL;
	}

	pmInfo->SetIStaticPortMapping(tempPortMapping);
	pmInfo->Refresh();

	m_portMappingList.push_back(pmInfo);

	return pmInfo;
}

//==============================================================================
void CNatTraversal::RemovePortMapping(
	long int externalPort, 
	CString protocol)
{
	CError err;
	HRESULT hr = S_OK;

	std::vector<CPortMappingInfo*>::iterator it = m_portMappingList.begin();
	for (; it != m_portMappingList.end(); it++)
	{
		CPortMappingInfo* pmInfo = *it;

		if (externalPort == pmInfo->GetExternalPort()
			&& 0 == pmInfo->GetProtocol().Compare(protocol))
		{
			m_portMappingList.erase(it);
			break;
		}
	}

	hr = m_iPortMappingCollection->Remove(externalPort, protocol.GetBuffer());

	protocol.ReleaseBuffer();
}

//==============================================================================
