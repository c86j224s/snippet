#include "stdafx.h"
#include "PortMappingInfo.h"

//==============================================================================
CPortMappingInfo::CPortMappingInfo()
	: m_iStaticPortMapping(NULL)
	, m_enabled(FALSE)
	, m_protocol(_T(""))
	, m_description(_T(""))
	, m_internalClient(_T(""))
	, m_internalPort(0)
	, m_externalIPAddress(_T(""))
	, m_externalPort(0)
{
	// do nothing here
}

//==============================================================================
CPortMappingInfo::~CPortMappingInfo()
{
	Clear();
}

//==============================================================================
void CPortMappingInfo::Clear()
{
	if (m_iStaticPortMapping != NULL)
	{
		m_iStaticPortMapping->Release();
		m_iStaticPortMapping = NULL;
	}

	m_enabled = FALSE;
	m_protocol = _T("");
	m_description = _T("");
	m_internalClient = _T("");
	m_internalPort = 0;
	m_externalIPAddress = _T("");
	m_externalPort = 0;
}

//==============================================================================
CError CPortMappingInfo::Refresh()
{
	// [FIXME] 중간에 한번이라도 실패했을 때에 대한 예외 처리 필요.
	if (m_iStaticPortMapping == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }

	HRESULT hr;

	BSTR bStr;
	VARIANT_BOOL variantBool;
	long int longVal;

	// enabled ?
	hr = m_iStaticPortMapping->get_Enabled(&variantBool);
	if (FAILED(hr)) { return CError(hr, __FILE__, __LINE__); }
	else
	{
		m_enabled = (VARIANT_FALSE != variantBool);
	}

	// protocol
	hr = m_iStaticPortMapping->get_Protocol(&bStr);
	if (FAILED(hr)) { return CError(hr, __FILE__, __LINE__); }
	else if (bStr == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }
	else
	{
		_bstr_t bStrT(bStr);
		m_protocol.Format(bStrT);

		SysFreeString(bStr);
		bStr = NULL;
	}

	// description
	hr = m_iStaticPortMapping->get_Description(&bStr);
	if (FAILED(hr)) { return CError(hr, __FILE__, __LINE__); }
	else if (bStr == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }
	else
	{
		_bstr_t bStrT(bStr);
		m_description.Format(bStrT);

		SysFreeString(bStr);
		bStr = NULL;
	}

	// internal client ip address
	hr = m_iStaticPortMapping->get_InternalClient(&bStr);
	if (FAILED(hr)) { return CError(hr, __FILE__, __LINE__); }
	else if (bStr == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }
	else
	{
		_bstr_t bStrT(bStr);
		m_internalClient.Format(bStrT);

		SysFreeString(bStr);
		bStr = NULL;
	}

	// internal port
	hr = m_iStaticPortMapping->get_InternalPort(&longVal);
	if (FAILED(hr)) { return CError(hr, __FILE__, __LINE__); }
	else
	{
		m_internalPort = longVal;
	}

	// external ip address
	hr = m_iStaticPortMapping->get_ExternalIPAddress(&bStr);
	if (FAILED(hr)) { return CError(hr, __FILE__, __LINE__); }
	else if (bStr == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }
	else
	{
		_bstr_t bStrT(bStr);
		m_externalIPAddress.Format(bStrT);

		SysFreeString(bStr);
		bStr = NULL;
	}

	// external port
	hr = m_iStaticPortMapping->get_ExternalPort(&longVal);
	if (FAILED(hr)) { return CError(hr, __FILE__, __LINE__); }
	else
	{
		m_externalPort = longVal;
	}

	return CError(hr, __FILE__, __LINE__);
}

//==============================================================================
CError CPortMappingInfo::SetEnabled(BOOL enabled)
{
	if (m_iStaticPortMapping == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }

	HRESULT hr = m_iStaticPortMapping->Enable(enabled == TRUE ? VARIANT_TRUE : VARIANT_FALSE);
	if (SUCCEEDED(hr))
	{
		m_enabled = enabled;
	}
	
	return CError(hr, __FILE__, __LINE__);
}

//==============================================================================
CError CPortMappingInfo::SetDescription(CString desc)
{
	if (m_iStaticPortMapping == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }

	HRESULT hr = m_iStaticPortMapping->EditDescription(desc.GetBuffer());
	if (SUCCEEDED(hr))
	{
		m_description = desc;
	}
	desc.ReleaseBuffer();

	return CError(hr, __FILE__, __LINE__);
}

//==============================================================================
CError CPortMappingInfo::SetInternalClient(CString internalClient)
{
	if (m_iStaticPortMapping == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }

	HRESULT hr = m_iStaticPortMapping->EditInternalClient(internalClient.GetBuffer());
	if (SUCCEEDED(hr))
	{
		m_internalClient = internalClient;
	}
	internalClient.ReleaseBuffer();

	return CError(hr, __FILE__, __LINE__);
}

//==============================================================================
CError CPortMappingInfo::SetInternalPort(long int port)
{
	if (m_iStaticPortMapping == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }

	HRESULT hr = m_iStaticPortMapping->EditInternalPort(port);
	if (SUCCEEDED(hr))
	{
		m_internalPort = port;
	}

	return CError(hr, __FILE__, __LINE__);
}

//==============================================================================
