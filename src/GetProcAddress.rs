use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryExA};
use std::ffi::CString;
use std::ptr;

pub fn get_proc_address() {
    let file_name = CString::new("TranslucentTb").unwrap();
    unsafe {
        let handle = LoadLibraryExA(
            file_name.as_ptr(),
            ptr::null_mut(),
            0x00000001, 
        );

        if handle.is_null() {
            println!("Failed to load library");
            return;
        }

        let function_name = CString::new("TranslucentTB").unwrap();
        let result = GetProcAddress(handle, function_name.as_ptr());

        if result.is_null() {
            println!("Failed to get procedure address");
        } 
    }
}
