#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str { // define the content of the index
    "Hello, world!"
}

#[launch]
fn rocket() -> _ { 
    rocket::build().mount("/", routes![index]) // builds and mounts the route to the index
}
