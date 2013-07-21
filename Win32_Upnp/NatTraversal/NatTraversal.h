#pragma once

//==============================================================================
class CNatTraversal
{
public:
	CNatTraversal();
	~CNatTraversal() {}

	CError Init();
	void Fin();

	CError Refresh();
	void Clear();

	int GetCount();
	
	CPortMappingInfo* GetPortMappingInfo(int idx);

	CPortMappingInfo* GetPortMappingInfo(
		long int externalPort, 
		CString protocol);

	CPortMappingInfo* AddPortMapping(
		long int externalPort, 
		CString protocol,
		long int internalPort,
		CString internalClient,
		BOOL enabled,
		CString description);

	void RemovePortMapping(
		long int externalPort, 
		CString protocol);

	/*
	long int GetPortMappingCount()
	{
		long int portMappingCount;

		if (NULL == m_iPortMappingCollection
			|| FAILED(m_iPortMappingCollection->get_Count(&portMappingCount)))
		{
			return -1;
		}

		return portMappingCount;
	}
	*/


/*
	CError GetPortMappingInfo(long int externalPort, CString protocol, struct CPortMappingInfo& portMappingInfo)
	{
		CError err;

		HRESULT hr = S_OK;
		IStaticPortMapping* staticPortMapping = NULL;

		hr = m_iPortMappingCollection->get_Item(externalPort, protocol.GetBuffer(), &staticPortMapping);
		if (FAILED(hr)) { return CError(hr, __FILE__, __LINE__); }
		else if (staticPortMapping == NULL) { return CError(S_FALSE, __FILE__, __LINE__); }
		else
		{
			err = GetPortMappingInfoFromIStaticPortMapping(staticPortMapping, portMappingInfo);

			staticPortMapping->Release();
			staticPortMapping = NULL;
		}

		protocol.ReleaseBuffer();

		return err;
	}

*/


private:
	std::vector<CPortMappingInfo*> m_portMappingList;

	IUPnPNAT* m_upnpNat;
	IStaticPortMappingCollection* m_iPortMappingCollection;
};

//==============================================================================