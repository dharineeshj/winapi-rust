
use winapi::um::processthreadsapi::{TerminateProcess, GetExitCodeProcess, OpenProcess};
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::{PROCESS_TERMINATE, PROCESS_QUERY_INFORMATION};

pub fn terminate_process(process_id: u32) -> Result<(), String> {
    unsafe {
        let process_handle = OpenProcess(PROCESS_TERMINATE | PROCESS_QUERY_INFORMATION, 0, process_id);
        if process_handle.is_null() {
            return Err("Failed to open process.".to_string());
        }
        let mut exit_code: u32 = 0;
        if GetExitCodeProcess(process_handle, &mut exit_code) == 0 {
            CloseHandle(process_handle);
            return Err("Failed to get process exit code.".to_string());
        }
        if exit_code == 259 { 
            if TerminateProcess(process_handle, 1) == 0 {
                CloseHandle(process_handle);
                return Err("Failed to terminate the process.".to_string());
            }
        }
        if CloseHandle(process_handle) == 0 {
            return Err("Failed to close process handle.".to_string());
        }
    }

    Ok(())
}
