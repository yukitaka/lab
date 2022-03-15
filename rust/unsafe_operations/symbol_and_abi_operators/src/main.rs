use std::arch::asm;

extern "C" fn foo(arg: i32) -> i32 {
    println!("arg = {}", arg);
    arg * 2
}

fn main() {
    println!("{}", call_foo(4));
}

fn call_foo(arg: i32) -> i32 {
    unsafe {
        let result;
        asm!(
            "call {}",
            in(reg) foo,
            in("rdi") arg,
            out("rax") result,
            clobber_abi("C"),
        );
        result
    }
}
