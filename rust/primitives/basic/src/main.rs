mod literals;

fn main() {
    let logical: bool = true;
    println!("{}", logical);

    let a_float: f64 = 1.0;
    let an_integer = 5i32;
    println!("{} {}", a_float, an_integer);

    let default_float = 3.0;
    let default_integer = 7;
    println!("{} {}", default_float, default_integer);

    let mut inferred_type = 12;
    println!("{}", inferred_type);
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    println!("{}", mutable);
    mutable = 21;
    println!("{} {}", inferred_type, mutable);

    let mutable = true;
    println!("{}", mutable);

    literals::literals();
}
