#![allow(dead_code, unused_variables)]
use async_trait::async_trait;

use crate::api::data::kitsu::{CoverImage, KitsuSearchData};
use crate::api::data::{ApiData, Data, Images, Titles};
use crate::api::{ApiImpl, Res};
use crate::unwrap;

pub struct KitsuResponse(String, String);

impl KitsuResponse {
    fn json(&self) -> (KitsuSearchData, KitsuSearchData) {
        let anime: KitsuSearchData = serde_json::from_str(self.0.as_str()).unwrap();
        let manga: KitsuSearchData = serde_json::from_str(self.1.as_str()).unwrap();
        (anime, manga)
    }
}

impl Into<ApiData> for KitsuResponse {
    fn into(self) -> ApiData {
        let (anime, manga) = self.json();

        ApiData {
            anime: Some(
                anime
                    .data
                    .unwrap()
                    .into_iter()
                    .map(|a| {
                        let attr = a.attributes.unwrap();
                        let titles = attr.titles.unwrap();
                        let cover_image = unwrap!(attr.cover_image, CoverImage::default()).unwrap();

                        Data {
                            id: a.id,
                            data_type: a.datum_type,
                            titles: Some(Titles {
                                en: titles.en,
                                jp: titles.ja_jp,
                                roman: titles.en_jp,
                            }),
                            canon_title: attr.canonical_title,
                            rating: attr.average_rating.into(),
                            popularity: attr.popularity_rank,
                            rank: attr.rating_rank,
                            age_rating: attr.age_rating,
                            age_rating_guide: attr.age_rating_guide,
                            sub_type: attr.subtype,
                            status: attr.status,
                            create_at: attr.created_at,
                            updated_at: attr.updated_at,
                            start_date: attr.start_date,
                            end_date: attr.end_date,
                            images: Some(Images {
                                tiny: cover_image.tiny,
                                small: cover_image.small,
                                medium: cover_image.original,
                                large: cover_image.large,
                            }),
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
                        let attr = a.attributes.unwrap();
                        let titles = attr.titles.unwrap();
                        let cover_image = attr.cover_image.unwrap_or(CoverImage::default());

                        Data {
                            id: a.id,
                            data_type: a.datum_type,
                            titles: Some(Titles {
                                en: titles.en,
                                jp: titles.ja_jp,
                                roman: titles.en_jp,
                            }),
                            canon_title: attr.canonical_title,
                            rating: attr.average_rating.into(),
                            popularity: attr.popularity_rank,
                            rank: attr.rating_rank,
                            age_rating: attr.age_rating,
                            age_rating_guide: attr.age_rating_guide,
                            sub_type: attr.subtype,
                            status: attr.status,
                            create_at: attr.created_at,
                            updated_at: attr.updated_at,
                            start_date: attr.start_date,
                            end_date: attr.end_date,
                            images: Some(Images {
                                tiny: cover_image.tiny,
                                small: cover_image.small,
                                medium: cover_image.original,
                                large: cover_image.large,
                            }),
                        }
                    })
                    .collect(),
            ),
        }
    }
}

pub struct KitsuApiImpl {
    url: String,
    client: reqwest::Client,
}

impl Default for KitsuApiImpl {
    fn default() -> Self {
        Self {
            url: "https://kitsu.io/api/edge".to_owned(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl ApiImpl for KitsuApiImpl {
    async fn search(&self, query: String) -> Res<ApiData> {
        let anime = self.search_anime(query.to_owned()).await?;
        let manga = self.search_manga(query.to_owned()).await?;
        Ok(KitsuResponse(anime, manga).into())
    }

    async fn search_anime(&self, query: String) -> Res<String> {
        let url = format!("{}/anime?filter[text]={}", self.url, query);
        let result = self.client.get(url).send().await?.text().await?;

        Ok(result)
    }

    async fn search_manga(&self, query: String) -> Res<String> {
        let url = format!("{}/manga?filter[text]={}", self.url, query);
        let result = self.client.get(url).send().await?.text().await?;

        Ok(result)
    }

    fn name(&self) -> &str {
        "Kitsu.io"
    }

    fn desc(&self) -> &str {
        "Uses Kitsu.io's API to search data"
    }
}
