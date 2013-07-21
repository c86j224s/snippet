// ConsoleProject1.cpp : 콘솔 응용 프로그램에 대한 진입점을 정의합니다.
//

#include "stdafx.h"

//========================================================================

class NewThreadPool
{
public:
	NewThreadPool (void) {}
	virtual ~NewThreadPool (void) {}

	//------------------------------------------------------------------------

	_Success_(return) BOOL StartThreadPool (void)
	{
		InitializeThreadpoolEnvironment (&tpCallbackEnviron_);

		tpPoolPtr_ = CreateThreadpool (nullptr);
		if (tpPoolPtr_ == nullptr)
		{
			return FALSE;
		}

		SetThreadpoolThreadMaximum (tpPoolPtr_, 4);	// @TODO 4...
		if (SetThreadpoolThreadMinimum (tpPoolPtr_, 0) == FALSE)
		{
			CleanupThreadPool (TRUE);
			return FALSE;
		}

		SetThreadpoolCallbackPool (&tpCallbackEnviron_, tpPoolPtr_);

		tpCleanupGroupPtr_ = CreateThreadpoolCleanupGroup ();
		if (tpCleanupGroupPtr_ == nullptr)
		{
			CleanupThreadPool (TRUE);
			return FALSE;
		}

		SetThreadpoolCallbackCleanupGroup (&tpCallbackEnviron_, tpCleanupGroupPtr_, nullptr);

		return TRUE;
	}

	void CleanupThreadPool (BOOL isForce)
	{
		if (tpCleanupGroupPtr_)
		{
			CloseThreadpoolCleanupGroupMembers (tpCleanupGroupPtr_, isForce, nullptr);

			CloseThreadpoolCleanupGroup (tpCleanupGroupPtr_);
		}

		if (tpPoolPtr_)
		{
			CloseThreadpool (tpPoolPtr_);
		}

		DestroyThreadpoolEnvironment (&tpCallbackEnviron_);
	}
	
	//------------------------------------------------------------------------

	_Success_(return) BOOL SubmitWork (_In_ PTP_WORK_CALLBACK workCallback, _In_opt_ PVOID context, _In_opt_ BOOL isSync = FALSE, 
					 _Out_opt_ PTP_WORK* tpWorkPtrOut = nullptr)
	{
		PTP_WORK tpWorkPtr = nullptr;

		tpWorkPtr = CreateThreadpoolWork (workCallback, context, &tpCallbackEnviron_);
		if (tpWorkPtr == nullptr)
		{
			return FALSE;
		}

		SubmitThreadpoolWork (tpWorkPtr);

		if (isSync)
		{
			WaitForThreadpoolWorkCallbacks(tpWorkPtr, FALSE);
			tpWorkPtr = NULL;
		}
		
		if (tpWorkPtrOut != nullptr)
		{
			*tpWorkPtrOut = tpWorkPtr;
		}

		return TRUE;
	}

	void CancelWork (_In_ PTP_WORK tpWorkPtr)
	{
		if (tpWorkPtr != nullptr)
		{
			CloseThreadpoolWork (tpWorkPtr);
		}
	}

	//------------------------------------------------------------------------

	_Success_(return) BOOL SubmitTimer (_In_ PTP_TIMER_CALLBACK timerCallback, _In_opt_ PVOID context, _In_opt_ LPFILETIME dueTimePtr, 
					  _In_opt_ DWORD period = 0, _In_opt_ DWORD windowLength = 0, _In_opt_ BOOL isSync = FALSE,
					  _Out_opt_ PTP_TIMER* tpTimerPtrOut = nullptr)
	{
		PTP_TIMER tpTimerPtr = nullptr;

		tpTimerPtr = CreateThreadpoolTimer (timerCallback, context, &tpCallbackEnviron_);
		if (tpTimerPtr == nullptr)
		{
			return FALSE;
		}

		SetThreadpoolTimer (tpTimerPtr, dueTimePtr, period, windowLength);

		if (isSync)
		{
			WaitForThreadpoolTimerCallbacks (tpTimerPtr, FALSE);
			tpTimerPtr = NULL;
		}

		if (tpTimerPtrOut != nullptr)
		{
			*tpTimerPtrOut = tpTimerPtr;
		}
		
		return TRUE;
	}

	void CancelTimer (_In_ PTP_TIMER tpTimerPtr)
	{
		if (tpTimerPtr != nullptr)
		{
			CloseThreadpoolTimer (tpTimerPtr);
		}
	}

	//------------------------------------------------------------------------

	_Success_(return) BOOL SubmitWait (_In_ PTP_WAIT_CALLBACK waitCallback, _In_opt_ PVOID context, _In_ HANDLE handle, 
					 _In_opt_ LPFILETIME dueTimePtr, _In_opt_ BOOL isSync = FALSE, 
					 _Out_opt_ PTP_WAIT* tpWaitPtrOut = nullptr)
	{
		PTP_WAIT tpWaitPtr = nullptr;

		tpWaitPtr = CreateThreadpoolWait (waitCallback, context, &tpCallbackEnviron_);
		if (tpWaitPtr == nullptr)
		{
			return FALSE;
		}

		SetThreadpoolWait (tpWaitPtr, handle, dueTimePtr);

		if (isSync)
		{
			WaitForThreadpoolWaitCallbacks (tpWaitPtr, FALSE);
			tpWaitPtr = NULL;
		}
		
		if (tpWaitPtrOut != nullptr)
		{
			*tpWaitPtrOut = tpWaitPtr;
		}
		
		return TRUE;
	}

	void CancelWait (PTP_WAIT tpWaitPtr)
	{
		if (tpWaitPtr != nullptr)
		{
			CloseThreadpoolWait (tpWaitPtr);
		}
	}

	//------------------------------------------------------------------------

	_Success_(return) BOOL SubmitIo (_In_ PTP_WIN32_IO_CALLBACK ioCallback, _In_opt_ PVOID context, _In_ HANDLE handle, 
				   _Out_opt_ PTP_IO* tpIoPtrOut = nullptr)
	{
		PTP_IO tpIoPtr = nullptr;

		tpIoPtr = CreateThreadpoolIo (handle, ioCallback, context, &tpCallbackEnviron_);
		if (tpIoPtr == nullptr)
		{
			return FALSE;
		}

		StartThreadpoolIo (tpIoPtr);

		if (tpIoPtrOut != nullptr)
		{
			*tpIoPtrOut = tpIoPtr;
		}

		return TRUE;
	}

	void CancelIo (PTP_IO tpIoPtr)
	{
		if (tpIoPtr != nullptr)
		{
			CancelThreadpoolIo (tpIoPtr);
			
			CloseThreadpoolIo (tpIoPtr);
		}
	}

	//------------------------------------------------------------------------

private:
	TP_CALLBACK_ENVIRON tpCallbackEnviron_;
	PTP_POOL tpPoolPtr_;
	PTP_CLEANUP_GROUP tpCleanupGroupPtr_;
};

//========================================================================

class NewThreadPoolSocket
{
public:
	NewThreadPoolSocket (void) {}
	virtual ~NewThreadPoolSocket (void) {}

};

//========================================================================

int _tmain(int argc, _TCHAR* argv[])
{
	UNREFERENCED_PARAMETER (argc);
	UNREFERENCED_PARAMETER (argv);

	WSADATA wd = {0,};
	if (WSAStartup (MAKEWORD (2, 2), &wd) != 0)
	{
		std::cerr << "winsock err" << std::endl;
		return 0;
	}

	NewThreadPool tp;

	if (tp.StartThreadPool () == FALSE)
	{
		std::cerr << "tp err" << std::endl;
		return 0;
	}

	for (int i = 0; i < 100; i++)
	{
		tp.SubmitWork ([](PTP_CALLBACK_INSTANCE tpInstance, PVOID context, PTP_WORK tpWorkPtr)
		{
			UNREFERENCED_PARAMETER (tpInstance);
			UNREFERENCED_PARAMETER (tpWorkPtr);

			int v = reinterpret_cast<int>(context);
			std::cout << v << std::endl;
			return;
		}, reinterpret_cast<PVOID>(i));
	}

	for (int i = 0; i < 1; i++)
	{
		ULARGE_INTEGER ull;
		FILETIME due = {0,};

		ull.QuadPart = (ULONGLONG)(-3*1000*1000*10);	// 3sec
		due.dwHighDateTime = ull.HighPart;
		due.dwLowDateTime = ull.LowPart;
		
		tp.SubmitTimer ([](PTP_CALLBACK_INSTANCE tpInstance, PVOID context, PTP_TIMER tpTimerPtr)
		{
			UNREFERENCED_PARAMETER (tpInstance);
			UNREFERENCED_PARAMETER (context);
			UNREFERENCED_PARAMETER (tpTimerPtr);

			std::cout << "timer callback!!" << std::endl;
		}, nullptr, &due);
	}

	Sleep (4000);

	do
	{
		HANDLE eventHandle = nullptr;

		eventHandle = CreateEvent (nullptr, FALSE, FALSE, L"test event");
		if (eventHandle == nullptr)
		{
			std::cerr << "create event fail" << std::endl;
			return 0;
		}
		
		tp.SubmitWait ([](PTP_CALLBACK_INSTANCE tpInstance, PVOID context, PTP_WAIT tpWaitPtr, TP_WAIT_RESULT waitResult)
		{
			UNREFERENCED_PARAMETER (tpInstance);
			UNREFERENCED_PARAMETER (context);
			UNREFERENCED_PARAMETER (tpWaitPtr);
			UNREFERENCED_PARAMETER (waitResult);

			std::cout << "wait callback!!" << std::endl;
		}, nullptr, eventHandle, nullptr, FALSE);

		Sleep (2000);

		SetEvent (eventHandle);

		CloseHandle (eventHandle);
	}
	while (false);

	tp.CleanupThreadPool (FALSE);

	WSACleanup ();

	return 0;
}

