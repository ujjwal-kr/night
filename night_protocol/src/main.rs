#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    "Hello, world!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}