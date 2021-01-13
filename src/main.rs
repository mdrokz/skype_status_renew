extern crate winapi;

use std::ptr::null_mut as NULL;

use std::time::Duration;

use winapi::um::winuser::{FindWindowA, SetForegroundWindow, ShowWindow};
use winapi::um::winuser::{SW_MINIMIZE, SW_RESTORE};

use winapi::um::errhandlingapi::GetLastError;

use winapi::shared::windef::HWND__;

fn main() {
    let skype_window_name: Vec<u8> = "Skype\0".to_string().bytes().collect();

    unsafe {
        let name_ptr = skype_window_name.as_ptr() as *const i8;

        // println!("TEST {}", *(name_ptr));

        // println!("TEST 2 {:?}", skype_window_name);

        let skype_window = FindWindowA(NULL(), name_ptr);

        println!("ERROR CODE: {}", GetLastError());

        // SetForegroundWindow(skype_window);

        // ShowWindow(skype_window, SW_RESTORE);

        // ShowWindow(skype_window, SW_MINIMIZE);

        set_interval(open_skype,5000,skype_window);
    }
}

fn open_skype(skype_window: *mut HWND__) {
    unsafe {

        SetForegroundWindow(skype_window);
    
        ShowWindow(skype_window, SW_RESTORE);
    
        std::thread::sleep(Duration::from_millis(1000));
    
        ShowWindow(skype_window, SW_MINIMIZE);
    }
}

fn set_interval<F,T>(f: F, time: u64,value: T)
where
    F: Fn(T), T: Copy
{
    std::thread::sleep(Duration::from_millis(time));

    f(value);

    set_interval(f, time,value)
}
