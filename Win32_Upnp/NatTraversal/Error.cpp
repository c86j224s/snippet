#include "stdafx.h"

//==============================================================================
CError::CError()
	: m_errorResult(S_OK)
	, m_errorString(L"") 
{
	// do nothing here.
}

//==============================================================================
CError::CError(HRESULT result, const char* file, int line)
	: m_errorResult(result)
	, m_errorString(L"") 
{
	m_errorString.Format(L"%S%d", file, line);
}	

//==============================================================================
void CError::Set(HRESULT result, const char* file, int line)
{
	m_errorResult = result;
	m_errorString.Format(L"%S%d", file, line);
}

//==============================================================================