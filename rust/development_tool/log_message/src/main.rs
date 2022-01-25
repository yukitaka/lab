use env_logger::{Builder, Target};
use log;

mod console;
mod custom_logger;
mod unix_syslog;

fn main() {
    if let Err(e) = unix_syslog::log_to_the_unix_syslog() {
        println!("{}", e);
    }
    if let Err(e) = custom_logger::log_messages_with_a_custom_logger() {
        println!("{}", e);
    }
    Builder::new().target(Target::Stdout).init();

    log::error!("This error has been printed to Stdout");

    execute_query("DROP TABLE students");
    console::log_an_error_message_to_the_console();
}

fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}
