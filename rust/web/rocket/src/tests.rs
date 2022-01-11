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

#[test]
fn delay() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/delay/1").dispatch();
    assert_eq!(Status::Ok, response.status());
    assert_eq!("{ \"delay\": \"1s\" }", response.into_string().unwrap());
}
