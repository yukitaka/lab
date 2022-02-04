mod avoid_writing_and_reading;
mod calculate_file_sizes;
mod file_modified;
mod find_all_files;
mod find_duplicate;
mod find_loops;
mod memory_map;
mod read_lines;
mod traverse_directories;

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
    find_loops::find_loops_for_a_given_path();
    find_duplicate::recursively_find_duplicate_file_names();
    if let Err(e) = find_all_files::recursively_find_all_files_with_given_predicate() {
        println!("{}", e);
    }
    traverse_directories::traverse_directories_while_skipping_dotfiles();
    calculate_file_sizes::recursively_calculate_file_sizes_at_given_depth();
}
