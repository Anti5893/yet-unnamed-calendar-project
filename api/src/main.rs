#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str { // define the content of the index
    "Hello, world!"
}
#[get("/hi")]
fn hi() -> &'static str { // define the content of the page with ID "hi"
    "Howdy, hey"
}

#[launch]
fn rocket() -> _ { 
    rocket::build()
    .mount("/", routes![index, hi]) // builds and mounts the routes specified in the array
}
