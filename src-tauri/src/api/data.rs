pub mod jikan;
pub mod kitsu;

use serde::Serialize;

/// Modelled after Kitsu.io's API response
#[derive(Serialize, Default)]
pub struct ApiData {
    pub anime: Option<Vec<Data>>,
    pub manga: Option<Vec<Data>>,
}

#[derive(Serialize, Default)]
pub struct Data {
    pub id: Option<String>,
    pub data_type: Option<String>,
    pub titles: Option<Titles>,
    pub canon_title: Option<String>,
    pub rating: Option<String>,
    pub popularity: Option<i64>,
    pub rank: Option<i64>,
    pub age_rating: Option<String>,
    pub age_rating_guide: Option<String>,
    pub sub_type: Option<String>,
    pub status: Option<String>,
    pub create_at: Option<String>,
    pub updated_at: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub images: Option<Images>,
}

#[derive(Serialize, Default)]
pub struct Images {
    pub tiny: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
}

#[derive(Serialize, Default)]
pub struct Titles {
    pub en: Option<String>,
    pub jp: Option<String>,
    pub roman: Option<String>,
}
