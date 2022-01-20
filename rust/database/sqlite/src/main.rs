mod create_sqlite;
mod insert_data;

fn main() {
    match create_sqlite::create_a_sqlite_database() {
        Err(e) => println!("{}", e),
        _ => (),
    }
    match insert_data::insert_and_select_data() {
        Err(e) => println!("{}", e),
        _ => (),
    }
}
