mod percent_encode;

fn main() {
    if let Err(e) = percent_encode::percent_encode_a_string() {
        println!("{}", e);
    }
}
