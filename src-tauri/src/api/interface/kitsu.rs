use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct KitsuSearchData {
    pub data: Option<Vec<Datum>>,
    pub meta: Option<WelcomeMeta>,
    pub links: Option<WelcomeLinks>,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub datum_type: Option<String>,
    pub links: Option<DatumLinks>,
    pub attributes: Option<Attributes>,
    pub relationships: Option<HashMap<String, Relationship>>,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    pub slug: Option<String>,
    pub synopsis: Option<String>,
    #[serde(rename = "coverImageTopOffset")]
    pub cover_image_top_offset: Option<i64>,
    pub titles: Option<Titles>,
    #[serde(rename = "canonicalTitle")]
    pub canonical_title: Option<String>,
    #[serde(rename = "abbreviatedTitles")]
    pub abbreviated_titles: Option<Vec<String>>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<String>,
    #[serde(rename = "ratingFrequencies")]
    pub rating_frequencies: Option<HashMap<String, String>>,
    #[serde(rename = "userCount")]
    pub user_count: Option<i64>,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: Option<i64>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "popularityRank")]
    pub popularity_rank: Option<f64>,
    #[serde(rename = "ratingRank")]
    pub rating_rank: Option<f64>,
    #[serde(rename = "ageRating")]
    pub age_rating: Option<String>,
    #[serde(rename = "ageRatingGuide")]
    pub age_rating_guide: Option<String>,
    pub subtype: Option<String>,
    pub status: Option<String>,
    pub tba: Option<String>,
    #[serde(rename = "posterImage")]
    pub poster_image: Option<PosterImage>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<CoverImage>,
    #[serde(rename = "episodeCount")]
    pub episode_count: Option<i64>,
    #[serde(rename = "episodeLength")]
    pub episode_length: Option<i64>,
    #[serde(rename = "youtubeVideoId")]
    pub youtube_video_id: Option<String>,
    #[serde(rename = "showType")]
    pub show_type: Option<String>,
    pub nsfw: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CoverImage {
    pub tiny: Option<String>,
    pub small: Option<String>,
    pub large: Option<String>,
    pub original: Option<String>,
    pub meta: Option<CoverImageMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct CoverImageMeta {
    pub dimensions: Option<Dimensions>,
}

#[derive(Serialize, Deserialize)]
pub struct Dimensions {
    pub tiny: Option<Large>,
    pub small: Option<Large>,
    pub large: Option<Large>,
    pub medium: Option<Option<Large>>,
}

#[derive(Serialize, Deserialize)]
pub struct Large {
    pub width: Option<Option<serde_json::Value>>,
    pub height: Option<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct PosterImage {
    pub tiny: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub original: Option<String>,
    pub meta: Option<CoverImageMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct Titles {
    pub en: Option<String>,
    pub en_jp: Option<String>,
    pub ja_jp: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DatumLinks {
    #[serde(rename = "self")]
    pub links_self: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Relationship {
    pub links: Option<RelationshipLinks>,
}

#[derive(Serialize, Deserialize)]
pub struct RelationshipLinks {
    #[serde(rename = "self")]
    pub links_self: Option<String>,
    pub related: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct WelcomeLinks {
    pub first: Option<String>,
    pub prev: Option<String>,
    pub next: Option<String>,
    pub last: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct WelcomeMeta {
    pub count: Option<i64>,
}
