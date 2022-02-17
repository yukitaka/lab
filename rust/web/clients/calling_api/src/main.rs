mod query_api;

#[tokio::main]
async fn main() {
    if let Err(e) = query_api::query_the_github_api().await {
        println!("{}", e);
    }
}
