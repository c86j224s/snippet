#[cfg(windows)]
extern crate winapi;

#[cfg(not(windows))]
extern crate psutil;



pub trait ProcAccessor {
    type Item;

    fn enum_processes() -> std::io::Result<Vec<Self::Item>>;
    fn find_process_by_name(process_name : &str) -> Option<Self::Item>;

    fn get_process_name(&self) -> std::io::Result<String>;
}


pub mod sys {

    #[cfg(windows)]
    pub type Proc = crate::process_util::WinProc;
    #[cfg(not(windows))]
    pub type Proc = crate::process_util::LinProc;
}


#[cfg(windows)]
pub struct WinProc {
    pid : u32
}

#[cfg(windows)]
impl WinProc {
    fn str_to_wide(msg : &str) -> Vec<u16> {
        use std::os::windows::ffi::OsStrExt;

        std::ffi::OsStr::new(msg).encode_wide().chain(std::iter::once(0)).collect()
    }

    fn message_box(msg : &str) -> Result<(), std::io::Error> {
        use winapi::um::winuser::{MessageBoxW, MB_OK};

        let msg_wide = Self::str_to_wide(msg);
        let ret = unsafe {
            MessageBoxW(std::ptr::null_mut(), msg_wide.as_ptr(), msg_wide.as_ptr(), MB_OK)
        };

        if ret == 0 {
            Err(std::io::Error::last_os_error())
        }
        else {
            Ok(())
        }
    }
}


#[cfg(windows)]
impl ProcAccessor for WinProc {
    type Item = WinProc;

    fn enum_processes() -> std::io::Result<Vec<Self::Item>> {
        use winapi::shared::minwindef::DWORD;
        use winapi::um::psapi::EnumProcesses;

        let mut lpid_processes : [DWORD; 8192] = [0; 8192];
        let mut cb : DWORD = 8192 * std::mem::size_of::<DWORD>() as u32;

        let count = unsafe {
            let lpcb_needed : *mut DWORD = &mut cb;
            EnumProcesses(lpid_processes.as_mut_ptr(), cb, lpcb_needed as *mut DWORD)
        };
        if count == 0 {
            return Err(std::io::Error::last_os_error());
        }

        let mut ret : Vec<Self::Item> = Default::default();
        for process_id in lpid_processes.iter() {
            if *process_id != 0 {
                ret.push(WinProc{ pid: *process_id });
            }
        }

        Ok(ret)
    }

    
    fn get_process_name(&self) -> std::io::Result<String> {
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::psapi::{EnumProcessModules, GetModuleBaseNameW};
        use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
        use winapi::shared::minwindef::{FALSE, HMODULE, DWORD};
        use winapi::shared::ntdef::NULL;

        // open process handle
        let h_process = unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, self.pid)
        };
        if h_process == NULL {
            return Err(std::io::Error::last_os_error());
        }

        // find a module from process
        /*
        let mut h_module : HMODULE = std::ptr::null_mut();
        let mut cb : DWORD = 1;

        let ret = unsafe {
            let cb_needed : *mut DWORD = &mut cb;
            EnumProcessModules(h_process, h_module as *mut HMODULE, cb, cb_needed as *mut DWORD)
        };
        if ret == FALSE {
            return Err(std::io::Error::last_os_error());
        }
        */

        // get base name
        let mut base_name : [u16; 1024] = [0; 1024];

        let name_len = unsafe {
            //GetModuleBaseNameW(h_process, h_module, base_name.as_mut_ptr() as *mut u16, 1024)
            GetModuleBaseNameW(h_process, std::ptr::null_mut() as HMODULE, base_name.as_mut_ptr() as *mut u16, 1024)
        };
        if name_len == 0 {
            return Err(std::io::Error::last_os_error());
        }

        Ok(String::from_utf16_lossy(&base_name[0..name_len as usize]))
    }

    fn find_process_by_name(process_name : &str) -> Option<Self::Item> {
        let proc_vec = match Self::enum_processes() {
            Err(_) => { return None; },
            Ok(v) => v
        };

        for proc in proc_vec {
            let pname = match proc.get_process_name() {
                Err(_) => { continue },
                Ok(s) => s.to_ascii_lowercase()
            };

            if pname == process_name.to_ascii_lowercase() {
                return Some(proc);
            }
        }

        None
    }
}


#[cfg(not(windows))]
pub struct LinProc {
    process: psutil::process::Process
}

#[cfg(not(windows))]
impl ProcAccessor for LinProc {
    type Item = LinProc;

    fn enum_processes() -> std::io::Result<Vec<Self::Item>> {
        use psutil::process;
        let mut all_procs : Vec<psutil::process::Process> = psutil::process::all()?;

        let mut ret : Vec<Self::Item> = Default::default();
        for process in all_procs {
            ret.push(LinProc { process: process });
        }

        Ok(ret)
    }

    fn find_process_by_name(process_name : &str) -> Option<Self::Item> {
        use psutil::process;
        let mut all_procs : Vec<psutil::process::Process> = match psutil::process::all() {
            Ok(procs) => procs,
            Err(e) => return None
        };

        for process in all_procs {
            if process.comm.to_ascii_lowercase() == process_name.to_ascii_lowercase() {
                return Some(LinProc { process : process });
            }
        }

        None
    }

    fn get_process_name(&self) -> std::io::Result<String> {
        Ok(self.process.comm.clone())
    }
}

