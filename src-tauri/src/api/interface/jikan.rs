use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct JikanAnimeSearchData {
    pub data: Option<Vec<AnimeData>>,
    pub pagination: Option<Pagination>,
}

#[derive(Serialize, Deserialize)]
pub struct JikanMangaSearchData {
    pub data: Option<Vec<MangaData>>,
    pub pagination: Option<Pagination>,
}

#[derive(Serialize, Deserialize)]
pub struct AnimeData {
    pub mal_id: Option<i64>,
    pub url: Option<String>,
    pub images: Option<HashMap<String, Image>>,
    pub trailer: Option<Trailer>,
    pub approved: Option<bool>,
    pub titles: Option<Vec<Title>>,
    pub title: Option<String>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub datum_type: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<i64>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    pub aired: Option<Aired>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<i64>,
    pub scored_by: Option<i64>,
    pub rank: Option<f64>,
    pub popularity: Option<f64>,
    pub members: Option<i64>,
    pub favorites: Option<i64>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<i64>,
    pub broadcast: Option<Broadcast>,
    pub producers: Option<Vec<Demographic>>,
    pub licensors: Option<Vec<Demographic>>,
    pub studios: Option<Vec<Demographic>>,
    pub genres: Option<Vec<Demographic>>,
    pub explicit_genres: Option<Vec<Demographic>>,
    pub themes: Option<Vec<Demographic>>,
    pub demographics: Option<Vec<Demographic>>,
}

#[derive(Serialize, Deserialize)]
pub struct MangaData {
    pub mal_id: Option<i64>,
    pub url: Option<String>,
    pub images: Option<HashMap<String, Image>>,
    pub approved: Option<bool>,
    pub titles: Option<Vec<Title>>,
    pub title: Option<String>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    #[serde(rename = "type")]
    pub datum_type: Option<String>,
    pub chapters: Option<i64>,
    pub volumes: Option<i64>,
    pub status: Option<String>,
    pub publishing: Option<bool>,
    pub published: Option<Published>,
    pub score: Option<i64>,
    pub scored_by: Option<i64>,
    pub rank: Option<i64>,
    pub popularity: Option<i64>,
    pub members: Option<i64>,
    pub favorites: Option<i64>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub authors: Option<Vec<Author>>,
    pub serializations: Option<Vec<Author>>,
    pub genres: Option<Vec<Author>>,
    pub explicit_genres: Option<Vec<Author>>,
    pub themes: Option<Vec<Author>>,
    pub demographics: Option<Vec<Author>>,
}

#[derive(Serialize, Deserialize)]
pub struct Aired {
    pub from: Option<String>,
    pub to: Option<String>,
    pub prop: Option<Prop>,
}

#[derive(Serialize, Deserialize)]
pub struct Prop {
    pub from: Option<From>,
    pub to: Option<From>,
    pub string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct From {
    pub day: Option<i64>,
    pub month: Option<i64>,
    pub year: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Broadcast {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
    pub string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Demographic {
    pub mal_id: Option<i64>,
    #[serde(rename = "type")]
    pub demographic_type: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "type")]
    pub title_type: Option<String>,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Trailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub last_visible_page: Option<i64>,
    pub has_next_page: Option<bool>,
    pub items: Option<Items>,
}

#[derive(Serialize, Deserialize)]
pub struct Items {
    pub count: Option<i64>,
    pub total: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub mal_id: Option<i64>,
    #[serde(rename = "type")]
    pub author_type: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Published {
    pub from: Option<String>,
    pub to: Option<String>,
    pub prop: Option<Prop>,
}
