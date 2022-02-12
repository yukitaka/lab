mod hashtags;
mod login;

fn main() {
    login::verify_and_extract_login_from_an_email_address();
    hashtags::extract_a_list_of_unique_hashtags_from_a_text();
}
