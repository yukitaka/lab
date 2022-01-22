mod create_table;
mod insert_data;

fn main() {
    let _ = create_table::create_tables_in_a_postgres_database();
    match insert_data::insert_and_query_data() {
        Err(e) => println!("{}", e),
        _ => (),
    }
}
