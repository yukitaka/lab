mod create_sqlite;

fn main() {
    match create_sqlite::create_a_sqlite_database() {
        Err(e) => println!("{}", e),
        _ => (),
    }
}
