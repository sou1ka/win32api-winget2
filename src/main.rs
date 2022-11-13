use std::ffi::OsString;
use windows::{
    Win32::Foundation::{HWND, PWSTR, RECT},
    Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowRect},
};

fn main() {
    let id: HWND = unsafe { FindWindowW(PWSTR::default(), OsString::from("Windows PowerShell")) };
    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
    unsafe { 
        GetWindowRect(id, &mut rect);
    }
    println!("{:?}", rect);
}
