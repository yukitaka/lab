mod get_request;

fn main() {
    if let Err(e) = get_request::make_a_http_get_request() {
        println!("{}", e);
    }
}
