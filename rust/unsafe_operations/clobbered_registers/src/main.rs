use std::arch::asm;

fn main() {
    let mut ebx: u32;
    let mut edx: u32;
    let mut ecx: u32;
    unsafe {
        asm!("push rbx", "cpuid", "mov {0:e}, ebx","pop rbx", out(reg) ebx, out("edx") edx, out("ecx") ecx, inout("eax") 0 => _,);
    }

    let mut s = String::with_capacity(12);
    ebx.to_ne_bytes().map(|b| s.push(char::from(b)));
    edx.to_ne_bytes().map(|b| s.push(char::from(b)));
    ecx.to_ne_bytes().map(|b| s.push(char::from(b)));
    println!("CPU Manufacturer ID: {}", s);
}
