#pragma once

//==============================================================================
class CError
{
public:
	CError();
	CError(HRESULT result, const char* file, int line);
	~CError() {}

	HRESULT GetResult()		{ return m_errorResult; }
	CString& GetString()	{ return m_errorString; }

	void Set(HRESULT result, const char* file, int line);

private:
	HRESULT m_errorResult;
	CString m_errorString;
};

//==============================================================================