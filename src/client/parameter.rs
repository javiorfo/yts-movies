use crate::Genre;

#[derive(Debug)]
pub struct Filters(Filter);

impl Default for Filters {
    /// Creates a `Filters` builder with default parameters.
    fn default() -> Self {
        Self(Filter {
            quality: Quality::All,
            genre: Genre::All,
            rating: Rating::All,
            year: Year::All,
            order_by: OrderBy::Latest,
            page: 1,
        })
    }
}

impl Filters {
    /// Sets the quality filter.
    pub fn quality(mut self, quality: Quality) -> Self {
        self.0.quality = quality;
        self
    }

    /// Sets the genre filter.
    pub fn genre(mut self, genre: Genre) -> Self {
        self.0.genre = genre;
        self
    }

    /// Sets the rating filter.
    pub fn rating(mut self, rating: Rating) -> Self {
        self.0.rating = rating;
        self
    }

    /// Sets the year filter.
    pub fn year(mut self, year: Year) -> Self {
        self.0.year = year;
        self
    }

    /// Sets the sorting order.
    pub fn order_by(mut self, order_by: OrderBy) -> Self {
        self.0.order_by = order_by;
        self
    }

    /// Sets the page.
    pub fn page(mut self, page: u32) -> Self {
        self.0.page = page;
        self
    }

    /// Builds and returns the configured [`Filter`].
    pub fn build(self) -> Filter {
        self.0
    }
}

#[derive(Debug)]
pub struct Filter {
    pub quality: Quality,
    pub genre: Genre,
    pub rating: Rating,
    pub year: Year,
    pub order_by: OrderBy,
    pub page: u32,
}

impl Filter {
    pub fn quality_to_str(&self) -> &str {
        let quality = &self.quality;
        quality.into()
    }

    pub fn genre_to_str(&self) -> &str {
        let genre = &self.genre;
        genre.into()
    }

    pub fn rating_to_str(&self) -> &str {
        let rating = &self.rating;
        rating.into()
    }

    pub fn year_to_str(&self) -> String {
        let year = &self.year;
        year.into()
    }

    pub fn order_by_to_str(&self) -> &str {
        let order_by = &self.order_by;
        order_by.into()
    }
}

#[derive(Debug)]
pub enum Quality {
    All,
    P720,
    P1080,
    P2160,
    ThreeD,
}

impl From<&Quality> for &str {
    fn from(value: &Quality) -> Self {
        match value {
            Quality::All => "all",
            Quality::P720 => "720p",
            Quality::P1080 => "1080p",
            Quality::P2160 => "2160p",
            Quality::ThreeD => "3D",
        }
    }
}

impl From<&str> for Quality {
    fn from(value: &str) -> Self {
        if value.contains("720") {
            return Self::P720;
        }
        if value.contains("1080") {
            return Self::P1080;
        }
        if value.contains("2160") {
            return Self::P2160;
        }
        Self::ThreeD
    }
}

#[derive(Debug)]
pub enum Rating {
    All,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl From<&Rating> for &str {
    fn from(value: &Rating) -> Self {
        match value {
            Rating::All => "0",
            Rating::One => "1",
            Rating::Two => "2",
            Rating::Three => "3",
            Rating::Four => "4",
            Rating::Five => "5",
            Rating::Six => "6",
            Rating::Seven => "7",
            Rating::Eight => "8",
            Rating::Nine => "9",
        }
    }
}

#[derive(Debug)]
pub enum Year {
    All,
    Equal(u32),
    Range2000to2009,
    Range1990to1999,
    Range1980to1989,
    Range1970to1979,
    Range1950to1969,
    Range1900to1949,
}

impl From<&Year> for String {
    fn from(value: &Year) -> Self {
        match value {
            Year::All => "0".to_string(),
            Year::Equal(year) => year.to_string(),
            Year::Range2000to2009 => "2000-2009".to_string(),
            Year::Range1990to1999 => "1990-1999".to_string(),
            Year::Range1980to1989 => "1980-1989".to_string(),
            Year::Range1970to1979 => "1970-1979".to_string(),
            Year::Range1950to1969 => "1950-1969".to_string(),
            Year::Range1900to1949 => "1900-1949".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum OrderBy {
    Latest,
    Oldest,
    Featured,
    Year,
    Rating,
    Likes,
    Alphabetical,
}

impl From<&OrderBy> for &str {
    fn from(value: &OrderBy) -> Self {
        match value {
            OrderBy::Latest => "latest",
            OrderBy::Oldest => "oldest",
            OrderBy::Featured => "featured",
            OrderBy::Year => "year",
            OrderBy::Rating => "rating",
            OrderBy::Likes => "likes",
            OrderBy::Alphabetical => "alphabetical",
        }
    }
}
