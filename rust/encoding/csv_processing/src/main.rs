mod different_delimiter;
mod invalid_csv;
mod matching_a_predicate;
mod read_csv_records;
mod read_deserialize;
mod serialize_record;
mod serialize_record_using_serde;
mod transform_csv;

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
    if let Err(e) = invalid_csv::handle_invalid_csv_data_with_serde() {
        println!("{}", e);
    }
    if let Err(e) = serialize_record::serialize_records_to_csv() {
        println!("{}", e);
    }
    if let Err(e) = serialize_record_using_serde::serialize_records_to_csv_using_serde() {
        println!("{}", e);
    }
    if let Err(e) = transform_csv::transform_csv_column() {
        println!("{}", e);
    }
}
