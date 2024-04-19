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
    // current routing setup:
    // static is mounted
    // all css is stored in css folder
    // folder ie. Index represents a route
    // html in that folder is called index
    .mount("/", routes![clicked])
}
