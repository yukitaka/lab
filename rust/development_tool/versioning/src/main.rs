use semver::{BuildMetadata, Error, Prerelease, Version};

fn main() {
    if let Err(e) = parse_and_increment_a_version_string() {
        print!("{}", e);
    }
    if let Err(e) = parse_a_complex_version_string() {
        println!("{}", e);
    }
    if let Err(e) = check_if_given_version_is_prerelease() {
        println!("{}", e);
    }
}

fn parse_and_increment_a_version_string() -> Result<(), Error> {
    let mut parsed_version = Version::parse("0.2.6")?;

    assert_eq!(
        parsed_version,
        Version {
            major: 0,
            minor: 2,
            patch: 6,
            pre: Prerelease::EMPTY,
            build: BuildMetadata::EMPTY,
        }
    );

    parsed_version.patch += 1;
    assert_eq!(parsed_version.to_string(), "0.2.7");
    println!("New patch release: v{}", parsed_version);

    parsed_version.minor += 1;
    parsed_version.patch = 0;
    assert_eq!(parsed_version.to_string(), "0.3.0");
    println!("New minor release: v{}", parsed_version);

    parsed_version.major += 1;
    parsed_version.minor = 0;
    parsed_version.patch = 0;
    assert_eq!(parsed_version.to_string(), "1.0.0");
    println!("New major release: v{}", parsed_version);

    Ok(())
}

fn parse_a_complex_version_string() -> Result<(), Error> {
    let version_str = "1.0.49-125+g72ee7853";
    let parsed_version = Version::parse(version_str)?;

    assert_eq!(
        parsed_version,
        Version {
            major: 1,
            minor: 0,
            patch: 49,
            pre: Prerelease::new("125")?,
            build: BuildMetadata::new("g72ee7853")?,
        }
    );
    assert_eq!(parsed_version.build, BuildMetadata::new("g72ee7853")?,);

    let serialized_version = parsed_version.to_string();
    assert_eq!(&serialized_version, version_str);

    Ok(())
}

fn check_if_given_version_is_prerelease() -> Result<(), Error> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    let version_2 = Version::parse("1.0.0")?;

    assert_ne!(version_1.pre, Prerelease::EMPTY);
    assert_eq!(version_2.pre, Prerelease::EMPTY);

    Ok(())
}
