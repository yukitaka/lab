use std::arch::asm;

fn main() {
    println!("{}", mul(10, 20));
}

fn mul(a: u64, b: u64) -> u128 {
    let lo: u64;
    let hi: u64;

    unsafe {
        asm!(
            "mul {}",
            in(reg) a,
            inlateout("rax") b => lo,
            lateout("rdx") hi
        );
    }
    ((hi as u128) << 64) + lo as u128
}
