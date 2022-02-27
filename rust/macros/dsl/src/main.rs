macro_rules! calculate {
    (eval $e: expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

#[allow(clippy::erasing_op)]
fn main() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
