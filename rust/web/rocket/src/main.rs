#[macro_use]
extern crate rocket;

use rocket::http::{ContentType, Status};
use rocket::{Build, Rocket};

#[get("/")]
fn index() -> (Status, (ContentType, &'static str)) {
    (
        Status::ImATeapot,
        (ContentType::JSON, "{ \"hi\": \"world\" }"),
    )
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Oops! Rocket didn't launch!");
        drop(e);
    };
}

fn rocket() -> Rocket<Build> {
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
