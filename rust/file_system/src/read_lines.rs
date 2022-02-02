use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};

pub fn read_lines_of_strings_from_a_file() -> Result<(), Error> {
    let path = "lines.txt";
    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFunA")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
