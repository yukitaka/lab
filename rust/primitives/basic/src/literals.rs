#[allow(clippy::nonminimal_bool)]
pub fn literals() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    println!("not true is {}", !true);

    println!("0011 and 0101 is {0:04b} {0:#04x}", 0b0011u32 & 0b0101);
    println!("0011 or 0101 is {0:04b} {0:#04x}", 0b0011u32 | 0b0101);
    println!("0011 xor o1o1 is {0:04b} {0:#04x}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is {:#04x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}
