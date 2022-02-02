mod avoid_writing_and_reading;
mod read_lines;

fn main() {
    if let Err(e) = read_lines::read_lines_of_strings_from_a_file() {
        println!("{}", e);
    }
    if let Err(e) = avoid_writing_and_reading::avoid_writing_and_reading_from_a_same_file() {
        println!("{}", e);
    }
}
