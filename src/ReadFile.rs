use winapi::um::fileapi::{ReadFile, CreateFileA};
use winapi::um::winnt::{GENERIC_READ, FILE_ATTRIBUTE_NORMAL};
use std::ptr;
use std::ffi::CString;
use std::io::Error;

pub fn read_file() {
    let mut buffer: Vec<u8> = vec![0; 100]; // Buffer to read data into
    let mut bytes_read: u32 = 0; // Number of bytes read

    let filename = CString::new(r"E:\Projects\win32api\demo.txt").expect("CString::new failed");

    unsafe {
        // Open the file using CreateFileA
        let handle = CreateFileA(
            filename.as_ptr(),
            GENERIC_READ,
            0,
            ptr::null_mut(),
            3,
            FILE_ATTRIBUTE_NORMAL,
            ptr::null_mut(),
        );

        // Check if file handle is valid
        if handle.is_null() {
            eprintln!("Operation failed for CreateFileA: {:?}", Error::last_os_error());
            return;
        }

        // Read data from the file into the buffer
        let result = ReadFile(
            handle,
            buffer.as_mut_ptr() as *mut _,
            buffer.len() as u32,
            &mut bytes_read,
            ptr::null_mut(),
        );

        // Check if the read was successful
        if result == 0 {
            eprintln!("Operation failed for ReadFile: {:?}", Error::last_os_error());
        } else {
            // Convert buffer to string and print it
            let file_contents = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
            println!("File Contents: {}", file_contents);
        }
    }
}
