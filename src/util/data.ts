export interface ApiData {
    anime?: Data[]
    manga?: Data[]
}

interface Data {
    id: string
    data_type: string
    titles: Titles
    canon_title?: string
    rating: string
    popularity: number
    rank: number
    age_rating: string
    age_rating_guide: string
    sub_type: string
    status: string
    create_at: string
    updated_at: string
    start_date: string
    end_date: string
    images?: Images
}

interface Titles {
    en: string
    jp: string
    roman: string
}

interface Images {
    tiny?: string
    small?: string
    medium?: string
    large?: string
}