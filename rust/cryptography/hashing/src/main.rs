mod hashing;

fn main() {
    match hashing::hashing() {
        Err(e) => {
            for e in e.iter() {
                println!("{}", e);
            }
        }
        _ => (),
    }
}
