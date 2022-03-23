use std::env;

pub fn test_env() {
    println!("{}", get_env("HOGE", "FUGA"));
}

fn get_env(key: &str, default: &str) -> String {
    match env::var(key) {
        Ok(v) => v,
        Err(_) => default.to_string(),
    }
}