use std::mem::{size_of_val, MaybeUninit};

use winapi::um::errhandlingapi::GetLastError;

use winapi::shared::minwindef::DWORD;

fn enumerate() -> () {
    let mut a_processes: MaybeUninit<[DWORD; 1024]> = unsafe { std::mem::zeroed() };

    let mut t: MaybeUninit<DWORD> = unsafe { std::mem::zeroed() };

    let cb_needed: DWORD = 0;
    let c_processes: DWORD = 0;

    println!(
        "size: {}",
        size_of_val::<MaybeUninit<[DWORD; 1024]>>(&a_processes) as u32
    );

    unsafe {
        let p = a_processes.as_mut_ptr();

        let p2 = t.as_mut_ptr();

        let deref = *p;

        println!("pointer: {:p}", a_processes.as_mut_ptr() as *mut u32);

        println!("deref {:?}", deref);

        println!(
            "T: {:?}",
            EnumProcesses(
                a_processes.as_mut_ptr() as *mut u32,
                // size_of_val::<MaybeUninit<DWORD>>(&t) as u32,
                size_of_val::<MaybeUninit<[DWORD; 1024]>>(&a_processes) as u32,
                cb_needed as *mut u32
            )
        );

        println!("ERROR CODE: {}", GetLastError());
    }
}
