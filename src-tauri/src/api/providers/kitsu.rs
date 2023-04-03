#![allow(dead_code, unused_variables)]
use async_trait::async_trait;

use crate::api::{interface::kitsu::KitsuSearchData, ApiImpl, Res};

use super::{ApiData, Data, Images, Titles};

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
                    .into_iter()
                    .map(|a| Data {
                        id: a.id,
                        data_type: a.datum_type,
                        titles: Titles {
                            en: a.attributes.titles.en,
                            jp: a.attributes.titles.ja_jp,
                            roman: a.attributes.titles.en_jp,
                        },
                        canon_title: Some(a.attributes.canonical_title),
                        rating: a.attributes.average_rating.into(),
                        popularity: a.attributes.popularity_rank,
                        rank: a.attributes.rating_rank,
                        age_rating: Some(a.attributes.age_rating),
                        age_rating_guide: Some(a.attributes.age_rating_guide),
                        sub_type: Some(a.attributes.subtype),
                        status: a.attributes.status,
                        create_at: Some(a.attributes.created_at),
                        updated_at: Some(a.attributes.updated_at),
                        start_date: a.attributes.start_date,
                        end_date: a.attributes.end_date,
                        images: Some(Images {
                            tiny: Some(a.attributes.cover_image.tiny),
                            small: Some(a.attributes.cover_image.small),
                            medium: Some(a.attributes.cover_image.original),
                            large: Some(a.attributes.cover_image.large),
                        }),
                    })
                    .collect(),
            ),
            manga: Some(
                manga
                    .data
                    .into_iter()
                    .map(|a| Data {
                        id: a.id,
                        data_type: a.datum_type,
                        titles: Titles {
                            en: a.attributes.titles.en,
                            jp: a.attributes.titles.ja_jp,
                            roman: a.attributes.titles.en_jp,
                        },
                        canon_title: Some(a.attributes.canonical_title),
                        rating: a.attributes.average_rating.into(),
                        popularity: a.attributes.popularity_rank,
                        rank: a.attributes.rating_rank,
                        age_rating: Some(a.attributes.age_rating),
                        age_rating_guide: Some(a.attributes.age_rating_guide),
                        sub_type: Some(a.attributes.subtype),
                        status: a.attributes.status,
                        create_at: Some(a.attributes.created_at),
                        updated_at: Some(a.attributes.updated_at),
                        start_date: a.attributes.start_date,
                        end_date: a.attributes.end_date,
                        images: Some(Images {
                            tiny: Some(a.attributes.cover_image.tiny),
                            small: Some(a.attributes.cover_image.small),
                            medium: Some(a.attributes.cover_image.original),
                            large: Some(a.attributes.cover_image.large),
                        }),
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
