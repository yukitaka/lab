mod read_lines;

fn main() {
    if let Err(e) = read_lines::read_lines_of_strings_from_a_file() {
        println!("{}", e);
    }
}
