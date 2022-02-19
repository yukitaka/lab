mod download_temporary;
mod partial_download;
mod post_file;

fn main() {
    async_example();
    if let Err(e) = partial_download::make_a_partial_download_with_http_range_headers() {
        println!("{}", e);
    }
}

#[tokio::main]
async fn async_example() {
    if let Err(e) = download_temporary::download_a_file_to_a_temporary_directory().await {
        println!("{}", e);
    }
    if let Err(e) = post_file::post_a_file_to_pasters().await {
        println!("{}", e);
    }
}
