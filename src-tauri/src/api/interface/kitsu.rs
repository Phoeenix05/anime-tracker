use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct KitsuSearchData {
    pub data: Vec<Datum>,
    pub meta: WelcomeMeta,
    pub links: WelcomeLinks,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    pub id: String,
    #[serde(rename = "type")]
    pub datum_type: String,
    pub links: DatumLinks,
    pub attributes: Attributes,
    pub relationships: HashMap<String, Relationship>,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub slug: String,
    pub synopsis: String,
    #[serde(rename = "coverImageTopOffset")]
    pub cover_image_top_offset: i64,
    pub titles: Titles,
    #[serde(rename = "canonicalTitle")]
    pub canonical_title: String,
    #[serde(rename = "abbreviatedTitles")]
    pub abbreviated_titles: Vec<String>,
    #[serde(rename = "averageRating")]
    pub average_rating: String,
    #[serde(rename = "ratingFrequencies")]
    pub rating_frequencies: HashMap<String, String>,
    #[serde(rename = "userCount")]
    pub user_count: i64,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i64,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(rename = "popularityRank")]
    pub popularity_rank: i64,
    #[serde(rename = "ratingRank")]
    pub rating_rank: i64,
    #[serde(rename = "ageRating")]
    pub age_rating: String,
    #[serde(rename = "ageRatingGuide")]
    pub age_rating_guide: String,
    pub subtype: String,
    pub status: String,
    pub tba: String,
    #[serde(rename = "posterImage")]
    pub poster_image: PosterImage,
    #[serde(rename = "coverImage")]
    pub cover_image: CoverImage,
    #[serde(rename = "episodeCount")]
    pub episode_count: i64,
    #[serde(rename = "episodeLength")]
    pub episode_length: i64,
    #[serde(rename = "youtubeVideoId")]
    pub youtube_video_id: String,
    #[serde(rename = "showType")]
    pub show_type: String,
    pub nsfw: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CoverImage {
    pub tiny: String,
    pub small: String,
    pub large: String,
    pub original: String,
    pub meta: CoverImageMeta,
}

#[derive(Serialize, Deserialize)]
pub struct CoverImageMeta {
    pub dimensions: Dimensions,
}

#[derive(Serialize, Deserialize)]
pub struct Dimensions {
    pub tiny: Large,
    pub small: Large,
    pub large: Large,
    pub medium: Option<Large>,
}

#[derive(Serialize, Deserialize)]
pub struct Large {
    pub width: Option<serde_json::Value>,
    pub height: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct PosterImage {
    pub tiny: String,
    pub small: String,
    pub medium: String,
    pub large: String,
    pub original: String,
    pub meta: CoverImageMeta,
}

#[derive(Serialize, Deserialize)]
pub struct Titles {
    pub en: String,
    pub en_jp: String,
    pub ja_jp: String,
}

#[derive(Serialize, Deserialize)]
pub struct DatumLinks {
    #[serde(rename = "self")]
    pub links_self: String,
}

#[derive(Serialize, Deserialize)]
pub struct Relationship {
    pub links: RelationshipLinks,
}

#[derive(Serialize, Deserialize)]
pub struct RelationshipLinks {
    #[serde(rename = "self")]
    pub links_self: String,
    pub related: String,
}

#[derive(Serialize, Deserialize)]
pub struct WelcomeLinks {
    pub first: String,
    pub prev: String,
    pub next: String,
    pub last: String,
}

#[derive(Serialize, Deserialize)]
pub struct WelcomeMeta {
    pub count: i64,
}
