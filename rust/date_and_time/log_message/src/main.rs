mod console;

fn main() {
    env_logger::init();

    execute_query("DROP TABLE students");
    console::log_an_error_message_to_the_console();
}

fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}
