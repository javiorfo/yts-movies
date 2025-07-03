#[derive(Debug)]
pub struct Movie {
    pub name: String,
    pub year: u32,
    pub rating: f32,
    pub genre: Vec<Genre>,
    pub image: String,
    link: String,
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
