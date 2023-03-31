#![allow(unused_variables)]
use async_trait::async_trait;

use crate::api::Api;

const URL: &str = "https://api.jikan.moe/v4";

pub struct JikanApi;

#[async_trait]
impl Api for JikanApi {
    async fn search(&self, query: String) -> Result<Vec<String>, String> {
        let mut results = Vec::<String>::new();

        // Fetch data

        Ok(results)
    }

    async fn search_anime(&self, query: String) -> Result<Vec<String>, String> {
        let url = format!("{URL}/anime?q={query}");
        todo!()
    }

    async fn search_manga(&self, query: String) -> Result<Vec<String>, String> {
        let url = format!("{URL}/manga?q={query}");
        todo!()
    }
}
