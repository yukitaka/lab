mod broken_links;
mod extract_from_html;
mod extract_from_mediawiki;

#[tokio::main]
async fn main() {
    if let Err(e) = extract_from_html::extract_all_links_from_a_webpage_html().await {
        println!("{}", e);
    }
    if let Err(e) = broken_links::check_a_webpage_for_broken_links().await {
        println!("{}", e);
    }
    if let Err(e) = extract_from_mediawiki::extract_all_unique_links_from_a_mediawiki_markup().await
    {
        println!("{}", e);
    }
}
