#![allow(dead_code, unused_variables)]
use async_trait::async_trait;

use crate::api::data::jikan::{JikanAnimeSearchData, JikanMangaSearchData};
use crate::api::data::{ApiData, Data, Images, Titles};
use crate::api::{ApiImpl, Res};

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
                    .unwrap()
                    .into_iter()
                    .map(|a| {
                        let aired = a.aired.unwrap();
                        let images = a.images.unwrap();

                        Data {
                            id: Some(a.mal_id.unwrap().to_string()),
                            data_type: a.datum_type,
                            titles: Some(Titles {
                                en: a.title_english,
                                jp: a.title_japanese,
                                roman: a.title,
                            }),
                            popularity: a.popularity,
                            rank: a.rank,
                            age_rating: a.rating,
                            status: a.status,
                            start_date: aired.from,
                            end_date: aired.to,
                            images: Some(Images {
                                small: images.get("jpg").unwrap().small_image_url.clone(),
                                medium: images.get("jpg").unwrap().image_url.clone(),
                                large: images.get("jpg").unwrap().large_image_url.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }
                    })
                    .collect(),
            ),
            manga: Some(
                manga
                    .data
                    .unwrap()
                    .into_iter()
                    .map(|a| {
                        let published = a.published.unwrap();
                        let images = a.images.unwrap();

                        Data {
                            id: Some(a.mal_id.unwrap().to_string()),
                            data_type: a.datum_type,
                            titles: Some(Titles {
                                en: a.title_english,
                                jp: a.title_japanese,
                                roman: a.title,
                            }),
                            popularity: a.popularity,
                            rank: a.rank,
                            status: a.status,
                            start_date: published.from,
                            end_date: published.to,
                            images: Some(Images {
                                small: images.get("jpg").unwrap().small_image_url.clone(),
                                medium: images.get("jpg").unwrap().image_url.clone(),
                                large: images.get("jpg").unwrap().large_image_url.clone(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }
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
        "Uses Jikan.moe's 3rd party MyAnimeList API to search data"
    }
}
