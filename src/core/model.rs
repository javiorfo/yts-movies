#[derive(Debug)]
pub struct Movie {
    pub name: String,
    pub year: u32,
    pub rating: f32,
    pub genre: Vec<Genre>,
    pub image: String,
    pub(crate) link: String,
}

impl Movie {
    pub(crate) fn new(
        name: String,
        year: u32,
        rating: f32,
        genre: Vec<Genre>,
        image: String,
        link: String,
    ) -> Self {
        Self {
            name,
            year,
            rating,
            genre,
            image,
            link,
        }
    }
}

#[derive(Debug)]
pub enum Genre {
    All,
    Action,
    Adventure,
    Animation,
    Biography,
    Comedy,
    Crime,
    Documentary,
    Drama,
    Family,
    Fantasy,
    FilmNoir,
    GameShow,
    History,
    Horror,
    Music,
    Musical,
    Mystery,
    News,
    RealityTV,
    Romance,
    SciFi,
    Sport,
    TalkShow,
    Thriller,
    War,
    Western,
}

impl From<&Genre> for &str {
    fn from(value: &Genre) -> Self {
        match value {
            Genre::All => "all",
            Genre::Action => "action",
            Genre::Adventure => "adventure",
            Genre::Animation => "animation",
            Genre::Biography => "biography",
            Genre::Comedy => "comedy",
            Genre::Crime => "crime",
            Genre::Documentary => "documentary",
            Genre::Drama => "drama",
            Genre::Family => "family",
            Genre::Fantasy => "fantasy",
            Genre::FilmNoir => "film-noir",
            Genre::GameShow => "game-show",
            Genre::History => "history",
            Genre::Horror => "horror",
            Genre::Music => "music",
            Genre::Musical => "musical",
            Genre::Mystery => "mystery",
            Genre::News => "news",
            Genre::RealityTV => "reality-tv",
            Genre::Romance => "romance",
            Genre::SciFi => "sci-fi",
            Genre::Sport => "sport",
            Genre::TalkShow => "talk-show",
            Genre::Thriller => "thriller",
            Genre::War => "war",
            Genre::Western => "western",
        }
    }
}

impl From<&str> for Genre {
    fn from(value: &str) -> Self {
        match value {
            "Action" => Genre::Action,
            "Adventure" => Genre::Adventure,
            "Animation" => Genre::Animation,
            "Biography" => Genre::Biography,
            "Comedy" => Genre::Comedy,
            "Crime" => Genre::Crime,
            "Documentary" => Genre::Documentary,
            "Drama" => Genre::Drama,
            "Family" => Genre::Family,
            "Fantasy" => Genre::Fantasy,
            "Film-Noir" => Genre::FilmNoir,
            "Game-Show" => Genre::GameShow,
            "History" => Genre::History,
            "Horror" => Genre::Horror,
            "Music" => Genre::Music,
            "Musical" => Genre::Musical,
            "Mystery" => Genre::Mystery,
            "News" => Genre::News,
            "Reality-TV" => Genre::RealityTV,
            "Romance" => Genre::Romance,
            "Sci-Fi" => Genre::SciFi,
            "Sport" => Genre::Sport,
            "Talk-Show" => Genre::TalkShow,
            "Thriller" => Genre::Thriller,
            "War" => Genre::War,
            "Western" => Genre::Western,
            _ => unreachable!(),
        }
    }
}
