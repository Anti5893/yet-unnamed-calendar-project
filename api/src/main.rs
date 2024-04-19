#[macro_use] extern crate rocket;

#[get("/")] // A Rocket route attribute can be any one of get, put, post, delete, head, patch, or options
fn index() -> &'static str { // define the content of the index
    "Hello, wor"
}
#[get("/hi")]
fn hi() -> &'static str { // define the content of the page with ID "hi"
    "Howdy, hey"
}
#[get("/hello/<name>")] // display dynamic based on the route
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}
// the docs have this example, which is insane
#[get("/hai/<name>/<age>/<cool>")] // but we can just pass parameters in the URL
// might be worth considering, since the calendar won't need to be super secure
fn hai(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[launch]
fn rocket() -> _ { 
    rocket::build()
    .mount("/", routes![index, hi, hello, hai]) // builds and mounts the routes specified in the array
    // could also do
    // .mount("/", routes![index]) and
    // .mount("/", routes![hi])
    // to mount the routes individually on separate lines
}
