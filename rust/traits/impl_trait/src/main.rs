use std::io::{self, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("sample.csv")?;
    match parse_csv_document(BufReader::new(f)) {
        Ok(t) => println!("{:?}", t),
        Err(e) => println!("{}", e)
    }
    Ok(())
}

fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}