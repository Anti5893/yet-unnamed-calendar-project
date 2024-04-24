#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};

fn get_component(file_name: &str) -> String {
    let path: String = format!("components/{file_name}.html");
    return std::fs::read_to_string(path).expect("File doesn't exist");
}

#[post("/clicked")]
fn clicked() -> String {
    return get_component("buttonClicked").replace("{{message}}", "Garfield I dont like mondays");
}
#[get("/clicked_twice")]
fn clicked_twice() -> String {
    return get_component("extra");
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("/frontend")))
        // current routing setup:
        // static is mounted
        // all css is stored in css folder
        // folder ie. Index represents a route
        // html in that folder is called index
        .mount("/", routes![clicked, clicked_twice])
}
