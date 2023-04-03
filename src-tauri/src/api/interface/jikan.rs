use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct JikanAnimeSearchData {
    pub data: Vec<AnimeData>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct JikanMangaSearchData {
    pub data: Vec<MangaData>,
    pub pagination: Pagination,
}

#[derive(Serialize, Deserialize)]
pub struct AnimeData {
    pub mal_id: i64,
    pub url: String,
    pub images: HashMap<String, Image>,
    pub trailer: Trailer,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    pub title_synonyms: Vec<String>,
    #[serde(rename = "type")]
    pub datum_type: String,
    pub source: String,
    pub episodes: i64,
    pub status: String,
    pub airing: bool,
    pub aired: Aired,
    pub duration: String,
    pub rating: String,
    pub score: i64,
    pub scored_by: i64,
    pub rank: i64,
    pub popularity: i64,
    pub members: i64,
    pub favorites: i64,
    pub synopsis: String,
    pub background: String,
    pub season: String,
    pub year: i64,
    pub broadcast: Broadcast,
    pub producers: Vec<Demographic>,
    pub licensors: Vec<Demographic>,
    pub studios: Vec<Demographic>,
    pub genres: Vec<Demographic>,
    pub explicit_genres: Vec<Demographic>,
    pub themes: Vec<Demographic>,
    pub demographics: Vec<Demographic>,
}

#[derive(Serialize, Deserialize)]
pub struct MangaData {
    pub mal_id: i64,
    pub url: String,
    pub images: HashMap<String, Image>,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: String,
    pub title_japanese: String,
    #[serde(rename = "type")]
    pub datum_type: String,
    pub chapters: i64,
    pub volumes: i64,
    pub status: String,
    pub publishing: bool,
    pub published: Published,
    pub score: i64,
    pub scored_by: i64,
    pub rank: i64,
    pub popularity: i64,
    pub members: i64,
    pub favorites: i64,
    pub synopsis: String,
    pub background: String,
    pub authors: Vec<Author>,
    pub serializations: Vec<Author>,
    pub genres: Vec<Author>,
    pub explicit_genres: Vec<Author>,
    pub themes: Vec<Author>,
    pub demographics: Vec<Author>,
}

#[derive(Serialize, Deserialize)]
pub struct Aired {
    pub from: String,
    pub to: String,
    pub prop: Prop,
}

#[derive(Serialize, Deserialize)]
pub struct Prop {
    pub from: From,
    pub to: From,
    pub string: String,
}

#[derive(Serialize, Deserialize)]
pub struct From {
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Broadcast {
    pub day: String,
    pub time: String,
    pub timezone: String,
    pub string: String,
}

#[derive(Serialize, Deserialize)]
pub struct Demographic {
    pub mal_id: i64,
    #[serde(rename = "type")]
    pub demographic_type: String,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "type")]
    pub title_type: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Trailer {
    pub youtube_id: String,
    pub url: String,
    pub embed_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Pagination {
    pub last_visible_page: i64,
    pub has_next_page: bool,
    pub items: Items,
}

#[derive(Serialize, Deserialize)]
pub struct Items {
    pub count: i64,
    pub total: i64,
    pub per_page: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub mal_id: i64,
    #[serde(rename = "type")]
    pub author_type: String,
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Published {
    pub from: String,
    pub to: String,
    pub prop: Prop,
}
