use winapi::um::winuser::{CreateWindowExA, CW_USEDEFAULT};
use winapi::um::winuser::WS_OVERLAPPEDWINDOW;
use std::ptr::null_mut;
use winapi::um::libloaderapi::GetModuleHandleW;

pub fn create_Window() {
    unsafe {
        let hwnd = CreateWindowExA(
            0,
            b"Windows\0".as_ptr() as *const i8,
            b"Test Window\0".as_ptr() as *const i8,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT, CW_USEDEFAULT, 800, 800,
            null_mut(), null_mut(), GetModuleHandleW(null_mut()), null_mut(),
        );

        if hwnd.is_null() {
            println!("Failed to create window.");
        }
    }
}
