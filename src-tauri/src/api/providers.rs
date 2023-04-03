use serde::Serialize;

pub mod jikan;
pub mod kitsu;
pub mod offline;

/// Modelled after Kitsu.io's API response
#[derive(Serialize)]
pub struct ApiData {
    anime: Option<Vec<Data>>,
    manga: Option<Vec<Data>>,
}

#[derive(Serialize)]
pub struct Data {
    id: String,
    data_type: String,
    titles: Titles,
    canon_title: Option<String>,
    rating: Option<String>,
    popularity: i64,
    rank: i64,
    age_rating: Option<String>,
    age_rating_guide: Option<String>,
    sub_type: Option<String>,
    status: String,
    create_at: Option<String>,
    updated_at: Option<String>,
    start_date: String,
    end_date: String,
    images: Option<Images>,
}

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
