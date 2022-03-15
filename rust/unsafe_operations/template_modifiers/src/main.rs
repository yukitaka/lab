use std::arch::asm;

fn main() {
    register_template_modifiers();
    load_fpu_control_word(10);
    labels();
}

fn register_template_modifiers() {
    let mut x: u16 = 0xab;

    unsafe {
        asm!("mov {0:h}, {0:l}", inout(reg_abcd) x);
    }

    assert_eq!(x, 0xabab);
}

fn load_fpu_control_word(control: u16) {
    unsafe {
        asm!("fldcw [{}]", in(reg) &control, options(nostack));
    }
}

fn labels() {
    #[allow(unused_assignments)]
    let mut a = 0;
    unsafe {
        asm!(
            "mov {0:e}, 10",
            "2:",
            "sub {0:e}, 1",
            "cmp {0:e}, 3",
            "jle 2f",
            "jmp 2b",
            "2:",
            "add {0:e}, 2",
            out(reg) a,
        );
    }
    assert_eq!(a, 5);
}
