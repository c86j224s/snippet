#pragma once

//==============================================================================
class CPortMappingInfo
{
public:
	CPortMappingInfo();
	~CPortMappingInfo();

	void	Clear();

	void	SetIStaticPortMapping(IStaticPortMapping* iStaticPortMapping) { m_iStaticPortMapping = iStaticPortMapping; }
	CError	Refresh();
	
	CError		SetEnabled(BOOL enabled);
	BOOL		GetEnabled()	{	return m_enabled;	}
	
	CString		GetProtocol()	{	return m_protocol;	}

	CError		SetDescription(CString desc);
	CString		GetDescription()	{	return m_description;	}

	CError		SetInternalClient(CString internalClient);
	CString		GetInternalClient()	{	return m_internalClient;	}

	CError		SetInternalPort(long int port);
	long int	GetInternalPort()	{	return m_internalPort;	}

	CString		GetExternalIPAddress()	{	return m_externalIPAddress;	}

	long int	GetExternalPort()	{	return m_externalPort;	}

private:
	IStaticPortMapping*	m_iStaticPortMapping;

	BOOL		m_enabled;
	CString		m_protocol;
	CString		m_description;
	CString		m_internalClient;
	long int	m_internalPort;
	CString		m_externalIPAddress;
	long int	m_externalPort;
};

//==============================================================================