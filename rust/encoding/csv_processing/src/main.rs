mod different_delimiter;
mod matching_a_predicate;
mod read_csv_records;
mod read_deserialize;

fn main() {
    if let Err(e) = read_csv_records::read_csv_records() {
        println!("{}", e);
    }
    if let Err(e) = read_deserialize::read_csv_deserialize() {
        println!("{}", e);
    }
    if let Err(e) = different_delimiter::read_csv_records_with_different_delimiter() {
        println!("{}", e);
    }
    if let Err(e) = matching_a_predicate::filter_csv_records_matching_a_predicate() {
        println!("{}", e);
    }
}
