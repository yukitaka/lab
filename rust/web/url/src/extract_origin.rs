use url::{Host, ParseError, Url};

pub fn extract_the_url_origin() -> Result<(), ParseError> {
    let s = "ftp://rust-lang.org/examples";
    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    println!("The origin is as expected!");

    Ok(())
}
