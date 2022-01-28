mod percent_encode;
mod urlencoded;

fn main() {
    if let Err(e) = percent_encode::percent_encode_a_string() {
        println!("{}", e);
    }
    urlencoded::encode_a_string_as_application_s_www_form_urlencoded();
}
