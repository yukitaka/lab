use std::env;
use std::fs;
use std::io::Error;

pub fn read_environment_variable() -> Result<(), Error> {
    let config_path = env::var("CONFIG").unwrap_or("/etc/apache2/ports.conf".to_string());
    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);

    Ok(())
}
