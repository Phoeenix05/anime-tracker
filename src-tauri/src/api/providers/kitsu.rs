use std::slice::Iter;

use async_trait::async_trait;

use crate::api::{ApiImpl, Res};

use super::{ApiData, Data, Images, Titles};

fn parse(data_iter: Iter<serde_json::Value>) -> Vec<Data> {
    // // Data {}
    // todo!()
    data_iter
        .map(|a| {
            let attributes = a["attributes"].as_object().unwrap();
            let images = attributes["posterImage"].as_object().unwrap();

            Data {
                id: a["id"].as_str().unwrap().to_owned(),
                data_type: a["type"].as_str().unwrap().to_owned(),
                titles: (|| {
                    let titles = attributes["titles"].as_object().unwrap();

                    Titles {
                        en: "".to_owned(),
                        jp: titles["ja_jp"].as_str().unwrap().to_owned(),
                        roman: titles["en_jp"].as_str().unwrap().to_owned(),
                    }
                })(),
                canon_title: Some(
                    attributes["canonicalTitle"]
                        .as_str()
                        .unwrap_or("")
                        .to_owned(),
                ),
                rating: attributes["averageRating"].as_str().unwrap_or("").to_owned(),
                popularity: attributes["averageRating"].as_i64().unwrap_or(0),
                rank: attributes["ratingRank"].as_i64().unwrap_or(0),
                age_rating: attributes["ageRating"].as_str().unwrap_or("").to_owned(),
                age_rating_guide: attributes["ageRatingGuide"].as_str().unwrap_or("").to_owned(),
                sub_type: attributes["subtype"].as_str().unwrap_or("").to_owned(),
                status: attributes["status"].as_str().unwrap_or("").to_owned(),
                create_at: attributes["createdAt"].as_str().unwrap_or("").to_owned(),
                updated_at: attributes["updatedAt"].as_str().unwrap_or("").to_owned(),
                start_date: attributes["startDate"].as_str().unwrap_or("").to_owned(),
                end_date: attributes["endDate"].as_str().unwrap_or("").to_owned(),
                images: Some(Images {
                    tiny: Some(images["tiny"].as_str().unwrap_or("").to_owned()),
                    small: Some(images["small"].as_str().unwrap_or("").to_owned()),
                    medium: Some(images["medium"].as_str().unwrap_or("").to_owned()),
                    large: Some(images["large"].as_str().unwrap_or("").to_owned()),
                }),
            }
        })
        .collect()
}

impl From<KitsuResponse> for ApiData {
    fn from(value: KitsuResponse) -> Self {
        let anime_data: serde_json::Value = serde_json::from_str(value.0.as_str()).unwrap();
        let manga_data: serde_json::Value = serde_json::from_str(value.1.as_str()).unwrap();
        // let iter = anime_data["data"].as_array().unwrap().iter();

        Self {
            anime: Some(parse(anime_data["data"].as_array().unwrap().iter())),
            manga: Some(parse(manga_data["data"].as_array().unwrap().iter())),
        }
    }
}

pub struct KitsuResponse(String, String);

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
