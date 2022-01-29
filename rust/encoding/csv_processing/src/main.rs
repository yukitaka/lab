mod read_csv_records;
mod read_deserialize;

fn main() {
    if let Err(e) = read_csv_records::read_csv_records() {
        println!("{}", e);
    }
    if let Err(e) = read_deserialize::read_csv_deserialize() {
        println!("{}", e);
    }
}
