mod gist;
mod query_api;

#[tokio::main]
async fn main() {
    if let Err(e) = query_api::query_the_github_api().await {
        println!("{}", e);
    }
    if let Err(e) = query_api::check_if_an_api_resource_exists().await {
        println!("{}", e);
    }
    if let Err(e) = gist::create_and_delete_gist_with_github_api().await {
        println!("{}", e);
    }
}
