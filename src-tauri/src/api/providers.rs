use serde::Serialize;

pub mod jikan;
pub mod kitsu;
pub mod offline;

// use self::{kitsu::KitsuResponse, jikan::JikanResponse, offline::OfflineResponse};

/// Modelled after Kitsu.io's API response
#[derive(Serialize)]
pub struct ApiData {
    anime: Option<Vec<Data>>,
    manga: Option<Vec<Data>>,
}

#[derive(Serialize)]
pub struct Data {
    id: String,
    // #[serde(rename = "type")]
    data_type: String,
    titles: Titles,
    // #[serde(rename = "canonicalTitle")]
    canon_title: Option<String>,
    rating: String,
    popularity: i64,
    rank: i64,
    age_rating: String,
    age_rating_guide: String,
    sub_type: String,
    status: String,
    create_at: String,
    updated_at: String,
    start_date: String,
    end_date: String,
    images: Option<Images>,
}

// #[derive(Serialize)]
// pub struct MangaData {
//     id: String,
//     // #[serde(rename = "type")]
//     data_type: String,
//     title: Titles,
//     // #[serde(rename = "canonicalTitle")]
//     canon_title: Option<String>,
//     rating: String,
//     popularity: i64,
//     rank: i64,
//     age_rating: String,
//     age_rating_guide: String,
//     sub_type: String,
//     status: String,
//     create_at: String,
//     updated_at: String,
//     start_date: String,
//     end_date: String,
//     images: Images,
// }

#[derive(Serialize)]
pub struct Images {
    tiny: Option<String>,
    small: Option<String>,
    medium: Option<String>,
    large: Option<String>,
}

#[derive(Serialize)]
pub struct Titles {
    en: String,
    jp: String,
    roman: String,
}
