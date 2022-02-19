mod gist;
mod query_api;
mod restful_api;

fn main() {
    async_example();
    if let Err(e) = restful_api::consume_a_paginated_restful_api() {
        println!("{}", e);
    }
}

#[tokio::main]
async fn async_example() {
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
