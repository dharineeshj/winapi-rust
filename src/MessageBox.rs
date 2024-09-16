use winapi::um::winuser::MessageBoxA;
use std::ffi::CString;

pub fn Message_Box(){
  let lptext=CString::new("Message from hacker").unwrap();
  let lpcaption=CString::new("Hacked Message").unwrap();
  unsafe{
    let messagebox = MessageBoxA(
      std::ptr::null_mut(),
      lptext.as_ptr(),
      lpcaption.as_ptr(),
      0x00000004, 
    );

    match messagebox{
      IDYES =>println!("user clicked yes"),
      IDNO  =>println!("user clicked no"),
      _ => println!("unexpected result"), 
    }
}
}