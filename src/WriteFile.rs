use winapi::um::fileapi::{WriteFile, CreateFileA};
use winapi::um::winnt::{GENERIC_WRITE, FILE_ATTRIBUTE_NORMAL};
use std::ffi::CString;
use std::ptr;
use std::io::Error;

pub fn write_file() {
    let filename = CString::new(r"E:\Projects\winapi-rust\demo.txt").expect("CString::new failed");
    let write_data = CString::new("The text has been written").expect("CString::new failed");
    let buffer: Vec<u8> = write_data.as_bytes().to_vec(); 
    let mut bytes_written: u32 = 0;

    unsafe {
        let handle = CreateFileA(
            filename.as_ptr(),
            GENERIC_WRITE,
            0,
            ptr::null_mut(),
            4, 
            FILE_ATTRIBUTE_NORMAL,
            ptr::null_mut(),
        );

        if handle.is_null() {
            eprintln!("Error opening file: {:?}", Error::last_os_error());
            return;
        }
        
        let result = WriteFile(
            handle,
            buffer.as_ptr() as *const _,
            buffer.len() as u32,
            &mut bytes_written,
            ptr::null_mut(),
        );

        if result == 0 {
            println!("Error writing to file: {:?}", Error::last_os_error());
        } else {
            println!("Successfully wrote {} bytes to the file.", bytes_written);
        }
    }
}
