fn main() {
    env_logger::init();

    execute_query("DROP TABLE students");
}

fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}
