use base64::{decode, encode};
use error_chain::error_chain;
use std::str;

error_chain! {
    foreign_links {
        Base64(base64::DecodeError);
        Utf8Error(str::Utf8Error);
    }
}

pub fn encode_and_decode_base64() -> Result<()> {
    let hello = b"hello rustaceans";
    let encoded = encode(hello);
    let decoded = decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}
