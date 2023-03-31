use async_trait::async_trait;

use crate::api::ApiImpl;

pub struct KitsuApiImpl {
    url: String,
}

impl Default for KitsuApiImpl {
    fn default() -> Self {
        let url = "https://kitsu.io/api/edge".to_owned();
        Self { url }
    }
}

#[async_trait]
impl ApiImpl for KitsuApiImpl {
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
