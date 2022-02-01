use serde::Deserialize;
use std::collections::HashMap;
use toml::{de::Error, Value};

pub fn deserialize_a_toml_configuration_file() -> Result<(), Error> {
    let toml_content = r#"
        [package]
        name = "your_package"
        version = "0.1.0"
        authors = ["You! <you@example.org>"]

        [dependencies]
        serde = "1.0"
        "#;
    let package_info: Value = toml::from_str(toml_content)?;

    assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
    assert_eq!(
        package_info["package"]["name"].as_str(),
        Some("your_package")
    );
    Ok(())
}

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

pub fn toml_using_serde() -> Result<(), Error> {
    let toml_content = r#"
        [package]
        name = "your_package"
        version = "0.1.0"
        authors = ["You! <you@example.org>"]

        [dependencies]
        serde = "1.0"
        "#;
    let package_info: Config = toml::from_str(toml_content)?;

    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["serde"], "1.0");

    Ok(())
}
