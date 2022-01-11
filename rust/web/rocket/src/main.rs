#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

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
