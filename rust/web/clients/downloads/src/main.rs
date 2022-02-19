mod download_temporary;
mod post_file;

fn main() {
    async_example();
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
