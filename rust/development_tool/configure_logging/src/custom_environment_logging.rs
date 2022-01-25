use env_logger::Builder;
use std::env;

pub fn use_a_custom_environment_variable_to_set_up_logging() {
    Builder::new()
        .parse_env(&env::var("MY_APP_LOG").unwrap_or_default())
        .init();

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");
}
