use std::arch::asm;


/*
python -c "import donut;a=donut.create(file='kaboum_exe.exe',arch=2)"
*/

//#[cfg(target_os = "windows")]
fn main() {
    #[link_section = ".text"]
    //static SHELLCODE: [u8; 17367660] = *include_bytes!("../../kaboum_exe.exe.bin");
    static SHELLCODE: [u8; 17367660] = *include_bytes!("../../kaboum_exe.exe.bin");

    unsafe {
        asm!(
        "call {}",
        in(reg) SHELLCODE.as_ptr(),
        )
    }
}
