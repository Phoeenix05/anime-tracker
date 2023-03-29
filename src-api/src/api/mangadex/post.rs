#![allow(unused_variables)]
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Mangadex {
    url: String,
}

// impl Mangadex {
//     fn parse_url(&self) -> Vec<String> {
//         todo!()
//     }
// }

#[post("/manga", data = "<data>")]
pub async fn manga(data: Json<Mangadex>) -> Result<String, ()> {
    Ok(Status::Ok.to_string())
}

#[post("/chapter", data = "<data>")]
pub async fn chapter(data: Json<Mangadex>) -> Result<String, ()> {
    Ok(Status::Ok.to_string())
}
