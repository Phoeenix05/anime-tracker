#[allow(dead_code, unused_variables)]

use async_trait::async_trait;

use crate::api::ApiImpl;

pub struct OfflineImpl {
    path: String,
}

impl Default for OfflineImpl {
    fn default() -> Self {
        Self {
            path: "".to_owned(),
        }
    }
}

#[async_trait]
impl ApiImpl for OfflineImpl {
    async fn search(&self, query: String) -> Result<(String, String), reqwest::Error> {
        todo!()
    }

    async fn search_anime(&self, query: String) -> Result<String, reqwest::Error> {
        todo!()
    }

    async fn search_manga(&self, query: String) -> Result<String, reqwest::Error> {
        todo!()
    }

    fn name(&self) -> &str {
        "offline"
    }

    fn desc(&self) -> &str {
        "Uses static json file to show data. Note: Some data might not be available or there will only be partial data availabla."
    }
}
