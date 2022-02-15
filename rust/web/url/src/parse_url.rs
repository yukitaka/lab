use url::{ParseError, Url};

pub fn parse_a_url_from_a_string_to_a_url_type() -> Result<(), ParseError> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}
