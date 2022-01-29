mod read_csv_records;

fn main() {
    if let Err(e) = read_csv_records::read_csv_records() {
        println!("{}", e);
    }
}
