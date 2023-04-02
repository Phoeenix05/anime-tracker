export interface KitsuData {
    data:  Datum[];
    meta:  KitsuDataMeta;
    links: KitsuDataLinks;
}

export interface Datum {
    id:            string;
    type:          string;
    links:         DatumLinks;
    attributes:    Attributes;
    relationships: { [key: string]: Relationship };
}

export interface Attributes {
    createdAt:           string;
    updatedAt:           string;
    slug:                string;
    synopsis:            string;
    coverImageTopOffset: number;
    titles:              Titles;
    canonicalTitle:      string;
    abbreviatedTitles:   string[];
    averageRating:       string;
    ratingFrequencies:   { [key: string]: string };
    userCount:           number;
    favoritesCount:      number;
    startDate:           string;
    endDate:             string;
    popularityRank:      number;
    ratingRank:          number;
    ageRating:           string;
    ageRatingGuide:      string;
    subtype:             string;
    status:              string;
    tba:                 string;
    posterImage:         PosterImage;
    coverImage:          CoverImage;
    episodeCount:        number;
    episodeLength:       number;
    youtubeVideoId:      string;
    showType:            string;
    nsfw:                boolean;
}

export interface CoverImage {
    tiny:     string;
    small:    string;
    large:    string;
    original: string;
    meta:     CoverImageMeta;
}

export interface CoverImageMeta {
    dimensions: Dimensions;
}

export interface Dimensions {
    tiny:    Large;
    small:   Large;
    large:   Large;
    medium?: Large;
}

export interface Large {
    width:  null;
    height: null;
}

export interface PosterImage {
    tiny:     string;
    small:    string;
    medium:   string;
    large:    string;
    original: string;
    meta:     CoverImageMeta;
}

export interface Titles {
    en:    string;
    en_jp: string;
    ja_jp: string;
}

export interface DatumLinks {
    self: string;
}

export interface Relationship {
    links: RelationshipLinks;
}

export interface RelationshipLinks {
    self:    string;
    related: string;
}

export interface KitsuDataLinks {
    first: string;
    prev:  string;
    next:  string;
    last:  string;
}

export interface KitsuDataMeta {
    count: number;
}

export interface JikanData {
    data: Data[];
}

export interface Data {
    mal_id:           number;
    url:              string;
    images:           { [key: string]: Image };
    trailer?:         Trailer;
    approved:         boolean;
    titles:           Title[];
    title:            string;
    title_english:    string;
    title_japanese:   string;
    title_synonyms?:  string[];
    type:             string;
    source?:          string;
    episodes?:        number;
    chapters?:        number;
    volumes?:         number;
    status:           string;
    airing?:          boolean;
    aired?:           Aired;
    publishing?:      boolean;
    published?:       Published;
    duration:         string;
    rating?:          string;
    score:            number;
    scored_by:        number;
    rank:             number;
    popularity:       number;
    members:          number;
    favorites:        number;
    synopsis:         string;
    background:       string;
    season?:          string;
    year?:            number;
    broadcast?:       Broadcast;
    producers?:       Demographic[];
    licensors?:       Demographic[];
    studios?:         Demographic[];
    authors:          Author[];
    serializations:   Author[];
    genres:           Demographic[];
    explicit_genres:  Demographic[];
    themes:           Demographic[];
    demographics:     Demographic[];
}

export interface Author {
    mal_id: number;
    type:   string;
    name:   string;
    url:    string;
}

export interface Aired {
    from: string;
    to:   string;
    prop: Prop;
}

export interface Published {
    from: string;
    to:   string;
    prop: Prop;
}

export interface Prop {
    from:   From;
    to:     From;
    string: string;
}

export interface From {
    day:   number;
    month: number;
    year:  number;
}

export interface Broadcast {
    day:      string;
    time:     string;
    timezone: string;
    string:   string;
}

export interface Demographic {
    mal_id: number;
    type:   string;
    name:   string;
    url:    string;
}

export interface Image {
    image_url:       string;
    small_image_url: string;
    large_image_url: string;
}

export interface Title {
    type:  string;
    title: string;
}

export interface Trailer {
    youtube_id: string;
    url:        string;
    embed_url:  string;
}
