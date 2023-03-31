#![allow(unused_variables)]

use crate::api::Api;

const URL: &str = "https://kitsu.io/api/edge";

pub struct KitsuApi;

impl Api for KitsuApi {
    fn search(&self, query: String) -> String {
        unimplemented!()
    }

    fn search_anime(&self, query: String) -> String {
        let url = format!("{URL}/anime");
        todo!()
    }

    fn search_manga(&self, query: String) -> String {
        let url = format!("{URL}/manga");
        todo!()
    }
}
