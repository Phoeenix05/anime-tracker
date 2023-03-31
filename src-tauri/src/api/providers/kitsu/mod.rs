#![allow(unused_variables)]
use async_trait::async_trait;

use crate::api::Api;

const URL: &str = "https://kitsu.io/api/edge";

pub struct KitsuApi;

#[async_trait]
impl Api for KitsuApi {
    async fn search(&self, query: String) -> Result<Vec<String>, String> {
        unimplemented!()
    }

    async fn search_anime(&self, query: String) -> Result<Vec<String>, String> {
        let url = format!("{URL}/anime");
        todo!()
    }

    async fn search_manga(&self, query: String) -> Result<Vec<String>, String> {
        let url = format!("{URL}/manga");
        todo!()
    }
}
