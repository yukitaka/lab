mod encryption;
mod hashing;
mod hmac_digest;

fn main() {
    match hashing::hashing() {
        Err(e) => {
            for e in e.iter() {
                println!("{}", e);
            }
        }
        _ => (),
    }
    match hmac_digest::sign_and_verify_a_message_with_hmac_digest() {
        Err(e) => println!("{}", e),
        _ => (),
    }
    match encryption::salt_and_hash_a_password_with_pbkdf2() {
        Err(e) => println!("{}", e),
        _ => (),
    }
}
