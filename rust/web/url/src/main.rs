mod parse_url;
mod removing_path;

fn main() {
    if let Err(e) = parse_url::parse_a_url_from_a_string_to_a_url_type() {
        println!("{}", e);
    }
    if let Err(e) = removing_path::create_a_base_url_by_removing_path_segments() {
        println!("{}", e);
    }
}
