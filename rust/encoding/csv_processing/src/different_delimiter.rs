use csv::Error;
use csv::ReaderBuilder;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

pub fn read_csv_records_with_different_delimiter() -> Result<(), Error> {
    let data = "name\tplace\tid
Mark\tMelbourne\t46
Ashley\tZurich\t92";

    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());
    for result in reader.deserialize::<Record>() {
        if let Ok(Record { id, name, place }) = result {
            println!("{:?} {} {}", id, name, place);
        };
    }

    Ok(())
}
