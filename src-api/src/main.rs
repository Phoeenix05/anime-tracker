#[macro_use]
extern crate rocket;

#[get("/hallo")]
fn hallo() -> &'static str {
    "Hallo there!"
}

#[catch(404)]
fn not_found() -> &'static str {
    "404 Not Found"
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![hallo])
        .register("/", catchers![not_found])
}
