use rayon::prelude::*;

fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 2) + fib(n - 1),
    }
}

fn main() {
    [10, 15, 20, 25, 30]
        .par_iter()
        .for_each(|n| println!("{}", fib(*n)));
}
