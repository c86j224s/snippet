#[cfg(windows)]
extern crate winapi;

#[cfg(not(windows))]
extern crate psutil;


#[cfg(windows)]
pub fn str_to_wide(msg : &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;

    std::ffi::OsStr::new(msg).encode_wide().chain(std::iter::once(0)).collect()
}

#[cfg(windows)]
pub fn message_box(msg : &str) -> Result<(), std::io::Error> {
    use winapi::um::winuser::{MessageBoxW, MB_OK};

    let msg_wide = str_to_wide(msg);
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

#[cfg(windows)]
pub fn enum_processes() -> Result<Vec<u32>, std::io::Error> {
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

    let mut ret : Vec<u32> = Default::default();
    for process_id in lpid_processes.iter() {
        if *process_id != 0 {
            ret.push(*process_id);
        }
    }

    Ok(ret)
}


#[cfg(windows)]
pub fn get_process_name(process_id : u32) -> Result<std::string::String, std::io::Error> {
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::{EnumProcessModules, GetModuleBaseNameW};
    use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
    use winapi::shared::minwindef::{FALSE, HMODULE, DWORD};
    use winapi::shared::ntdef::NULL;

    // open process handle
    let h_process = unsafe {
        OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, process_id)
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

#[cfg(windows)]
pub fn find_process_id_by_name(process_name : &str) -> Option<u32> {
    let pid_vec = match enum_processes() {
        Err(_) => { return None; },
        Ok(v) => v
    };

    for pid in pid_vec {
        let pname = match get_process_name(pid) {
            Err(_) => { continue },
            Ok(s) => s.to_ascii_lowercase()
        };

        if pname == process_name.to_ascii_lowercase() {
            return Some(pid);
        }
    }

    None
}

#[cfg(not(windows))]
pub fn find_process_id_by_name(process_name : &str) -> Option<u32> {
    use psutil::process;
    let mut all_procs : Vec<psutil::process::Process> = match psutil::process::all() {
        Ok(procs) => procs,
        Err(e) => return None
    };

    for process in all_procs {
        if process.comm.to_ascii_lowercase() == process_name.to_ascii_lowercase() {
            return Some(process.pid as u32);
        }
    }

    None
}