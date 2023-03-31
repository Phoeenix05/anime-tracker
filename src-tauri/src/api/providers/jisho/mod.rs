#![allow(unused_variables)]

use crate::api::Api;

const URL: &str = "https://api.jikan.moe/v4";

pub struct JikanApi;

impl Api for JikanApi {
    fn search(&self, query: String) -> String {
        todo!()
    }

    fn search_anime(&self, query: String) -> String {
        let url = format!("{URL}/anime?q={query}");
        todo!()
    }

    fn search_manga(&self, query: String) -> String {
        let url = format!("{URL}/manga?q={query}");
        todo!()
    }
}
