use winapi::um::processthreadsapi::{CreateProcessA, PROCESS_INFORMATION, STARTUPINFOA};
use std::ffi::CString;
use std::ptr;
use std::mem;
use std::io;

pub fn create_process() {
    let process_name = CString::new(r"C:\Windows\System32\notepad.exe").unwrap();

    let mut process_info: PROCESS_INFORMATION = unsafe { mem::zeroed() };
    let mut startup_info: STARTUPINFOA = unsafe { mem::zeroed() };


    unsafe {
        let result = CreateProcessA(
            process_name.as_ptr(),           
            ptr::null_mut(),                 
            ptr::null_mut(),                 
            ptr::null_mut(),                 
            0,                               
            0x00000010,                      
            ptr::null_mut(),                 
            ptr::null_mut(),                 
            &mut startup_info,               
            &mut process_info                
        );

        if result == 0 {
            let error_code = io::Error::last_os_error().raw_os_error().unwrap_or(0);
            println!("Process creation failed with error code: {}", error_code);
        } else {
            println!("Process created successfully and window should be visible.");
        }
    }
}
