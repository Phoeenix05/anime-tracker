use async_trait::async_trait;

use crate::api::ApiImpl;

pub struct JikanApiImpl {
    url: String,
    client: reqwest::Client,
}

impl Default for JikanApiImpl {
    fn default() -> Self {
        Self {
            url: "https://api.jikan.moe/v4".to_owned(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl ApiImpl for JikanApiImpl {
    async fn search(&self, query: String) -> Result<(String, String), reqwest::Error> {
        let anime = self.search_anime(query.to_owned()).await?;
        let manga = self.search_manga(query.to_owned()).await?;
        Ok((anime, manga))
    }

    async fn search_anime(&self, query: String) -> Result<String, reqwest::Error> {
        let url = format!("{}/anime?q={}", self.url, query);
        let result = self
            .client
            .get(url)
            .send()
            .await?
            .text()
            .await?;

        Ok(result)
    }

    async fn search_manga(&self, query: String) -> Result<String, reqwest::Error> {
        let url = format!("{}/manga?q={}", self.url, query);
        let result = self
            .client
            .get(url)
            .send()
            .await?
            .text()
            .await?;

        Ok(result)
    }
}