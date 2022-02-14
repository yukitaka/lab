mod extract_from_html;

#[tokio::main]
async fn main() {
    if let Err(e) = extract_from_html::extract_all_links_from_a_webpage_html().await {
        println!("{}", e);
    }
}
