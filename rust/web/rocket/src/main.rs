#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use rocket::http::{ContentType, Status};
use rocket::tokio::time::{sleep, Duration};
use rocket::{Build, Rocket};

#[get("/")]
fn index() -> (Status, (ContentType, &'static str)) {
    (
        Status::ImATeapot,
        (ContentType::JSON, "{ \"hi\": \"world\" }"),
    )
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> (Status, (ContentType, String)) {
    sleep(Duration::from_secs(seconds)).await;
    let msg = format!("{{ \"delay\": \"{}s\" }}", seconds);
    (Status::Ok, (ContentType::JSON, msg))
}

#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Oops! Rocket didn't launch!");
        drop(e);
    };
}

fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, delay])
}
