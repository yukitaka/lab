#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};

#[get("/")]
fn index() -> (Status, (ContentType, &'static str)) {
    (
        Status::ImATeapot,
        (ContentType::JSON, "{ \"hi\": \"world\" }"),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[cfg(test)]
mod test {
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
}
