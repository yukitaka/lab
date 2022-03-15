use std::arch::asm;

fn main() {
    register_template_modifiers();
}

fn register_template_modifiers() {
    let mut x: u16 = 0xab;

    unsafe {
        asm!("mov {0:h}, {0:l}", inout(reg_abcd) x);
    }

    assert_eq!(x, 0xabab);
}
