#![allow(unused_variables)]
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Zoro {
    url: String,
    mal_id: String,
}

#[post("/", data = "<data>")]
pub async fn data(data: Json<Zoro>) -> Result<String, ()> {
    Ok(Status::Ok.to_string())
}
