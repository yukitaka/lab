use reqwest::{Client, Error};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

pub async fn query_the_github_api() -> Result<(), Error> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(reqwest::header::USER_AGENT, "rust example")
        .send()
        .await?;

    let users = response.json::<Vec<User>>().await?;
    println!("{:?}", users);
    Ok(())
}
