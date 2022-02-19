use error_chain::error_chain;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::env;

error_chain! {
    foreign_links {
        EnvVar(env::VarError);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Gist {
    id: String,
    html_url: String,
}

pub async fn create_and_delete_gist_with_github_api() -> Result<()> {
    let gh_user = env::var("GH_USER")?;
    let gh_pass = env::var("GH_PASS")?;

    let gist_body = json!({
        "description": "the description for this gist",
        "public": true,
        "files": {
            "main.rs": {
                "content": r#"fn main() { println!("hello world!");}"#
            }
        }
    });
    let request_url = "https://api.github.com/gists";
    let response = Client::new()
        .post(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .header(reqwest::header::USER_AGENT, "rust example")
        .json(&gist_body)
        .send()
        .await?;
    let gist: Gist = response.json().await?;
    println!("Created {:?}", gist);

    let request_url = format!("{}/{}", request_url, gist.id);
    let response = Client::new()
        .delete(&request_url)
        .basic_auth(gh_user, Some(gh_pass))
        .header(reqwest::header::USER_AGENT, "rust example")
        .send()
        .await?;

    println!(
        "Gist {} deleted! Status code: {}",
        gist.id,
        response.status()
    );
    Ok(())
}
