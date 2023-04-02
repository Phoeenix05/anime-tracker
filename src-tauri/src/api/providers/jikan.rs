use async_trait::async_trait;

use crate::api::{ApiImpl, Res};

use super::ApiData;

impl From<JikanResponse> for ApiData {
    fn from(value: JikanResponse) -> Self {
        // let json: serde_json::Value = serde_json::from
        
        todo!()
    }
}

pub struct JikanResponse(String, String);

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
    async fn search(&self, query: String) -> Res<ApiData> {
        let anime = self.search_anime(query.to_owned()).await?;
        let manga = self.search_manga(query.to_owned()).await?;
        Ok(JikanResponse(anime, manga).into())
    }

    async fn search_anime(&self, query: String) -> Res<String> {
        let url = format!("{}/anime?q={}", self.url, query);
        let result = self.client.get(url).send().await?.text().await?;

        Ok(result)
    }

    async fn search_manga(&self, query: String) -> Res<String> {
        let url = format!("{}/manga?q={}", self.url, query);
        let result = self.client.get(url).send().await?.text().await?;

        Ok(result)
    }

    fn name(&self) -> &str {
        "Jikan (3rd party MyAnimeList API)"
    }

    fn desc(&self) -> &str {
        "Uses Jikan's API to search data"
    }
}
