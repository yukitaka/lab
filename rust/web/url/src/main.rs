mod parse_url;

fn main() {
    if let Err(e) = parse_url::parse_a_url_from_a_string_to_a_rul_type() {
        println!("{}", e);
    }
}
