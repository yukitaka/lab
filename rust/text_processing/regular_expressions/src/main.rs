mod hashtags;
mod login;
mod phone;

fn main() {
    login::verify_and_extract_login_from_an_email_address();
    hashtags::extract_a_list_of_unique_hashtags_from_a_text();
    if let Err(e) = phone::extract_phone_numbers_from_text() {
        println!("{}", e);
    }
}
