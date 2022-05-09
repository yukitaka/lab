extern crate core;

use std::cmp::{max, min};
use clap::{Command, Arg};

fn main() {
    let matches = Command::new("GCD")
        .version("0.0.1")
        .arg(
            Arg::new("x")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("y")
                .takes_value(true)
                .required(true),
        ).get_matches();

    let x = extract(matches.value_of("x"));
    let y = extract(matches.value_of("y"));

    println!("gcd({}, {}) = {}", x, y, gcd(max(x, y), min(x, y)));
}

fn extract(a: Option<&str>) -> usize {
    match a {
        Some(r) => match r.parse::<usize>() {
            Ok(r) => r,
            _ => panic!("Parse error"),
        }
        _ => panic!("Nothing"),
    }
}

fn gcd(max: usize, min: usize) -> usize {
    let r = max % min;
    if r == 0 {
        return min
    }

    gcd(min, r)
}
