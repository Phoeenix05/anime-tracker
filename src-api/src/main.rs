mod api;

#[macro_use]
extern crate rocket;

// #[get("/<t>")]
// fn b(t: i32) -> String {
//     format!("i32: {}", t)
// }

// #[get("/<t>", rank = 2)]
// fn a(t: &str) -> String {
//     format!("String: {}", t)
// }

#[launch]
async fn rocket() -> _ {
    rocket::build()
        // .mount("/", routes![a, b])
        .mount("/api", api::mangadex::routes())
        .mount("/api", api::zoro::routes())
}
