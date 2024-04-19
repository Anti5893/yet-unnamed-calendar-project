#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};

#[post("/clicked")]
fn clicked() {
    println!("Button clicked");
}
#[launch]
fn rocket() -> _ { 
    rocket::build()
    .mount("/", FileServer::from(relative!("static")))
    .mount("/", routes![clicked])
}
