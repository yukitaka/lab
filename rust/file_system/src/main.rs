mod avoid_writing_and_reading;
mod file_modified;
mod memory_map;
mod read_lines;

fn main() {
    if let Err(e) = read_lines::read_lines_of_strings_from_a_file() {
        println!("{}", e);
    }
    if let Err(e) = avoid_writing_and_reading::avoid_writing_and_reading_from_a_same_file() {
        println!("{}", e);
    }
    if let Err(e) = memory_map::access_a_file_randomly_using_a_memory_map() {
        println!("{}", e);
    }
    if let Err(e) = file_modified::file_names_that_have_been_modified_in_the_last_24hours() {
        println!("{}", e);
    }
}
