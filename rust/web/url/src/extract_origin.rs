use url::{Host, Origin, ParseError, Url};

pub fn extract_the_url_origin() -> Result<(), ParseError> {
    let s = "ftp://rust-lang.org/examples";
    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    println!("The origin is as expected!");

    Ok(())
}

pub fn extract_the_url_origin2() -> Result<(), ParseError> {
    let s = "ftp://rust-lang.org/examples";
    let url = Url::parse(s)?;

    let expected_scheme = "ftp".to_owned();
    let expected_host = Host::Domain("rust-lang.org".to_owned());
    let expected_port = 21;
    let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

    let origin = url.origin();
    assert_eq!(origin, expected);
    println!("The origin is as expected!");

    Ok(())
}
