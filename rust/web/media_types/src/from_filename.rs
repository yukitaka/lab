use mime::Mime;

pub fn get_mime_type_from_filename() {
    let filenames = vec!["foobar.jpg", "foo.bar", "foobar.png"];
    for file in filenames {
        let mime = find_mimetype(&file.to_owned());
        println!("MIME for {}: {}", file, mime);
    }
}

fn find_mimetype(filename: &str) -> Mime {
    let parts: Vec<&str> = filename.split('.').collect();
    let res = match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            &_ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    };
    res
}
