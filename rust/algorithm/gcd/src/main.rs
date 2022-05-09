extern crate core;

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

    println!("gcd({}, {}) = {}", x, y, gcd(x, y));
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

fn gcd(x: usize, y: usize) -> usize {
    let r = x % y;
    println!("{} % {} = {}", x, y, r);
    if r == 0 {
        return y
    }

    gcd(y, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greatest_common_divisor() {
        assert_eq!(26, gcd(78, 52));
        assert_eq!(2, gcd(20, 2));
        assert_eq!(8, gcd(24, 16));
    }

    #[test]
    fn test_unordered_number() {
        assert_eq!(26, gcd(52, 78));
        assert_eq!(2, gcd(2, 20));
        assert_eq!(8, gcd(16, 8));
    }

    #[test]
    fn test_triple_number() {
        assert_eq!(26, gcd(gcd(52, 78), 104));
        assert_eq!(2, gcd(gcd(2, 20), 90));
        assert_eq!(8, gcd(gcd(16, 8), 88));
    }
}
