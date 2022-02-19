use reqwest::{ClientBuilder, Result};
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct User {
    login: String,
    id: u32,
}

pub async fn query_the_github_api() -> Result<()> {
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

pub async fn check_if_an_api_resource_exists() -> Result<()> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client
        .head(&request_url)
        .header(reqwest::header::USER_AGENT, "rust example")
        .send()
        .await?;

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}
