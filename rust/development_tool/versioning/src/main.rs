use error_chain::error_chain;
use semver::{BuildMetadata, Prerelease, Version, VersionReq};
use std::process::Command;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Utf8(std::string::FromUtf8Error);
        Semver(semver::Error);
    }
}

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
    if let Err(e) = find_the_latest_version_satisfying_given_range() {
        println!("{}", e);
    }
    if let Err(e) = check_external_command_version_for_compatibility() {
        println!("{}", e);
    }
}

fn parse_and_increment_a_version_string() -> Result<()> {
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

fn parse_a_complex_version_string() -> Result<()> {
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

fn check_if_given_version_is_prerelease() -> Result<()> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    let version_2 = Version::parse("1.0.0")?;

    assert_ne!(version_1.pre, Prerelease::EMPTY);
    assert_eq!(version_2.pre, Prerelease::EMPTY);

    Ok(())
}

fn find_the_latest_version_satisfying_given_range() -> Result<()> {
    assert_eq!(
        find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"])?,
        Some(Version::parse("1.0.0")?)
    );

    assert_eq!(
        find_max_matching_version(
            ">1.2.3-alpha.3",
            vec![
                "1.2.3-alpha.3",
                "1.2.3-alpha.4",
                "1.2.3-alpha.10",
                "1.2.3-beta.4",
                "3.4.5-alpha.9",
            ]
        )?,
        Some(Version::parse("1.2.3-beta.4")?)
    );

    Ok(())
}

fn find_max_matching_version<'a, I>(version_req_str: &str, iterable: I) -> Result<Option<Version>>
where
    I: IntoIterator<Item = &'a str>,
{
    let vreq = VersionReq::parse(version_req_str)?;

    Ok(iterable
        .into_iter()
        .filter_map(|s| Version::parse(s).ok())
        .filter(|s| vreq.matches(s))
        .max())
}

fn check_external_command_version_for_compatibility() -> Result<()> {
    let version_constraint = "> 1.12.0";
    let version_test = VersionReq::parse(version_constraint)?;
    let output = Command::new("git").arg("--version").output()?;

    if !output.status.success() {
        error_chain::bail!("Command executed with failing error code");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let version = stdout
        .trim()
        .split(" ")
        .last()
        .ok_or_else(|| "Invalid command output")?;
    let parsed_version = Version::parse(version)?;

    if !version_test.matches(&parsed_version) {
        error_chain::bail!(
            "Command version lower than minimum supported version (found {}, need {})",
            parsed_version,
            version_constraint
        );
    }

    Ok(())
}
