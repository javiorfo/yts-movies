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

    /// Builds and returns the configured [`Filter`].
    pub fn build(self) -> Filter {
        self.0
    }
}

#[derive(Debug)]
pub struct Filter {
    quality: Quality,
    genre: Genre,
    rating: Rating,
    year: Year,
    order_by: OrderBy,
}

#[derive(Debug)]
pub enum Quality {
    All,
    P720,
    P1080,
    P2160,
    ThreeD,
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

#[derive(Debug)]
pub enum Year {
    All,
    Exact(u32),
    Range2000to2009,
    Range1990to1999,
    Range1980to1989,
    Range1970to1979,
    Range1950to1969,
    Range1900to1949,
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
