mod from_filename;
mod from_http_response;
mod from_string;

#[tokio::main]
async fn main() {
    from_string::get_mime_type_from_string();
    from_filename::get_mime_type_from_filename();
    if let Err(e) = from_http_response::parse_the_mime_type_of_a_http_response().await {
        println!("{}", e);
    }
}
