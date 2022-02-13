mod hashtags;
mod login;
mod multiple_regular_expressions;
mod phone;

fn main() {
    login::verify_and_extract_login_from_an_email_address();
    hashtags::extract_a_list_of_unique_hashtags_from_a_text();
    if let Err(e) = phone::extract_phone_numbers_from_text() {
        println!("{}", e);
    }
    if let Err(e) =
        multiple_regular_expressions::filter_a_log_file_by_matching_multiple_regular_expressions()
    {
        println!("{}", e);
    }
}
