mod from_filename;
mod from_string;

fn main() {
    from_string::get_mime_type_from_string();
    from_filename::get_mime_type_from_filename();
}
