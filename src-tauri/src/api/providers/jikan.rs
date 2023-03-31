use async_trait::async_trait;

use crate::api::ApiImpl;

pub struct JikanApiImpl {
    url: String,
}

impl Default for JikanApiImpl {
    fn default() -> Self {
        let url = "https://api.jikan.moe/v4".to_owned();
        Self { url }
    }
}

#[async_trait]
impl ApiImpl for JikanApiImpl {
    async fn search(&self, query: String) -> Result<String, String> {
        todo!()
    }

    async fn search_anime(&self, query: String) -> Result<String, String> {
        todo!()
    }

    async fn search_manga(&self, query: String) -> Result<String, String> {
        todo!()
    }
}
