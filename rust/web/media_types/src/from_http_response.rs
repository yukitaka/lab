use error_chain::error_chain;
use mime::Mime;
use reqwest::header::CONTENT_TYPE;
use std::str::FromStr;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Header(reqwest::header::ToStrError);
        Mime(mime::FromStrError);
    }
}

pub async fn parse_the_mime_type_of_a_http_response() -> Result<()> {
    let response = reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png").await?;
    let headers = response.headers();

    match headers.get(CONTENT_TYPE) {
        None => {
            println!("The response does not contain a Content-Type header.");
        }
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str()?)?;
            let media_type = match (content_type.type_(), content_type.subtype()) {
                (mime::TEXT, mime::HTML) => "a HTML document",
                (mime::TEXT, _) => "a text document",
                (mime::IMAGE, mime::PNG) => "a PNG image",
                (mime::IMAGE, _) => "an image",
                _ => "neither text nor image",
            };
            println!("The response contains {}.", media_type);
        }
    }

    Ok(())
}
