use winapi::um::fileapi::CreateFileA;
use winapi::um::winnt::GENERIC_READ;
use winapi::um::winnt::GENERIC_WRITE;
use std::ffi::CString;
use std::ptr;
pub fn create_windows_file() {
    let filename = CString::new("demo.txt").expect("CString::new failed");
    unsafe {
        let handle = CreateFileA(
            filename.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            0, 
            ptr::null_mut(),
            4,
            128,
            ptr::null_mut(),
        );

        if handle.is_null() {
            eprintln!("Failed to create or open the file.");
        } else {
            println!("File handle: {:?}", handle);
        }
    }
}
