/// Represents a movie with its basic attributes.
///
/// # Fields
/// - `name`: The title of the movie.
/// - `year`: The release year of the movie.
/// - `rating`: The movie's rating (e.g., IMDb score).
/// - `genre`: A list of genres associated with the movie.
/// - `image`: URL to the movie's poster or image.
/// - `link`: URL to more information about the movie.
///   This field is visible only within the current crate.
#[derive(Debug)]
pub struct Movie {
    /// The title of the movie.
    pub name: String,
    /// The release year of the movie.
    pub year: u32,
    /// The movie's rating (e.g., IMDb score).
    pub rating: f32,
    /// A list of genres associated with the movie.
    pub genre: Vec<Genre>,
    /// URL or path to the movie's poster or image.
    pub image: String,
    /// URL to more information about the movie (e.g., IMDb page).
    /// This field is `pub(crate)`, so it is accessible only within the current crate.
    pub(crate) link: String,
}

impl Movie {
    /// Creates a new `Movie` instance.
    ///
    /// # Parameters
    /// - `name`: The title of the movie.
    /// - `year`: The release year.
    /// - `rating`: The movie's rating.
    /// - `genre`: A vector of genres associated with the movie.
    /// - `image`: URL to the movie's image.
    /// - `link`: URL to more information about the movie.
    ///
    /// # Returns
    /// A new `Movie` instance with the specified attributes.
    ///
    /// # Visibility
    /// This constructor is `pub(crate)`, meaning it is public within the current crate only.
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

/// Represents the genre of a movie.
///
/// This enum covers a wide range of genres, including common and niche categories.
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
    /// Converts a reference to a `Genre` enum variant into its corresponding lowercase string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// let genre = Genre::Action;
    /// let genre_str: &str = (&genre).into();
    /// assert_eq!(genre_str, "action");
    /// ```
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
    /// Converts a string slice into a `Genre` enum variant.
    ///
    /// The input string should match the genre name with exact casing or hyphenation as specified.
    ///
    /// # Panics
    ///
    /// This function will panic if the input string does not correspond to any known genre.
    ///
    /// # Examples
    ///
    /// ```
    /// let genre = Genre::from("Action");
    /// assert_eq!(genre, Genre::Action);
    /// ```
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
            _ => panic!("Invalid genre"),
        }
    }
}
