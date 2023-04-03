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
    id: Option<String>,
    data_type: Option<String>,
    titles: Option<Titles>,
    canon_title: Option<String>,
    rating: Option<String>,
    popularity: Option<i64>,
    rank: Option<i64>,
    age_rating: Option<String>,
    age_rating_guide: Option<String>,
    sub_type: Option<String>,
    status: Option<String>,
    create_at: Option<String>,
    updated_at: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
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
    en: Option<String>,
    jp: Option<String>,
    roman: Option<String>,
}
