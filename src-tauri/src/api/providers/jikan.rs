#![allow(dead_code, unused_variables)]
use async_trait::async_trait;

use crate::api::{
    interface::jikan::{JikanAnimeSearchData, JikanMangaSearchData},
    ApiImpl, Res,
};

use super::{ApiData, Data, Images, Titles};

pub struct JikanResponse(String, String);

impl JikanResponse {
    fn json(&self) -> (JikanAnimeSearchData, JikanMangaSearchData) {
        let anime: JikanAnimeSearchData = serde_json::from_str(self.0.as_str()).unwrap();
        let manga: JikanMangaSearchData = serde_json::from_str(self.1.as_str()).unwrap();
        (anime, manga)
    }
}

impl Into<ApiData> for JikanResponse {
    fn into(self) -> ApiData {
        let (anime, manga) = self.json();

        ApiData {
            anime: Some(
                anime
                    .data
                    .into_iter()
                    .map(|a| Data {
                        id: a.mal_id.to_string(),
                        data_type: a.datum_type,
                        titles: Titles {
                            en: a.title_english,
                            jp: a.title_japanese,
                            roman: a.title,
                        },
                        canon_title: None,
                        rating: None,
                        popularity: a.popularity,
                        rank: a.rank,
                        age_rating: Some(a.rating),
                        age_rating_guide: None,
                        sub_type: None,
                        status: a.status,
                        create_at: None,
                        updated_at: None,
                        start_date: a.aired.from,
                        end_date: a.aired.to,
                        images: Some(Images {
                            tiny: None,
                            small: Some(a.images.get("jpg").unwrap().small_image_url.clone()),
                            medium: Some(a.images.get("jpg").unwrap().image_url.clone()),
                            large: Some(a.images.get("jpg").unwrap().large_image_url.clone()),
                        }),
                    })
                    .collect(),
            ),
            manga: Some(
                manga
                    .data
                    .into_iter()
                    .map(|a| Data {
                        id: a.mal_id.to_string(),
                        data_type: a.datum_type,
                        titles: Titles {
                            en: a.title_english,
                            jp: a.title_japanese,
                            roman: a.title,
                        },
                        canon_title: None,
                        rating: None,
                        popularity: a.popularity,
                        rank: a.rank,
                        age_rating: None,
                        age_rating_guide: None,
                        sub_type: None,
                        status: a.status,
                        create_at: None,
                        updated_at: None,
                        start_date: a.published.from,
                        end_date: a.published.to,
                        images: Some(Images {
                            tiny: None,
                            small: Some(a.images.get("jpg").unwrap().small_image_url.clone()),
                            medium: Some(a.images.get("jpg").unwrap().image_url.clone()),
                            large: Some(a.images.get("jpg").unwrap().large_image_url.clone()),
                        }),
                    })
                    .collect(),
            ),
        }
    }
}

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
