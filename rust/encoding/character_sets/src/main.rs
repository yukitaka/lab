mod encode_and_decode_base64;
mod encode_and_decode_hex;
mod percent_encode;
mod urlencoded;

fn main() {
    if let Err(e) = percent_encode::percent_encode_a_string() {
        println!("{}", e);
    }
    urlencoded::encode_a_string_as_application_s_www_form_urlencoded();
    if let Err(e) = encode_and_decode_hex::encode_and_decode_hex() {
        println!("{}", e);
    }
    if let Err(e) = encode_and_decode_base64::encode_and_decode_base64() {
        println!("{}", e);
    }
}
