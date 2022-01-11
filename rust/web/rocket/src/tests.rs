use super::rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(Status::ImATeapot, response.status());
    assert_eq!("{ \"hi\": \"world\" }", response.into_string().unwrap());
}
