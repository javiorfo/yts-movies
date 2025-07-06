use crate::Genre;

/// Builder pattern struct for configuring movie filters.
///
/// Wraps a [`Filter`] struct and provides a fluent interface to set filtering options.
///
/// # Examples
///
/// ```
/// let filters = Filters::default()
///     .quality(Quality::P1080)
///     .genre(Genre::Action)
///     .rating(Rating::Seven)
///     .year(Year::Range2000to2009)
///     .order_by(OrderBy::Rating)
///     .page(2)
///     .build();
/// ```
#[derive(Debug)]
pub struct Filters(Filter);

impl Default for Filters {
    /// Creates a `Filters` builder with default filter parameters.
    ///
    /// Defaults:
    /// - quality: `Quality::All`
    /// - genre: `Genre::All`
    /// - rating: `Rating::All`
    /// - year: `Year::All`
    /// - order_by: `OrderBy::Latest`
    /// - page: 1
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

/// Represents the finalized set of filters applied to movie queries.
///
/// This struct contains all filter parameters as concrete values.
#[derive(Debug)]
pub struct Filter {
    /// Quality filter.
    pub quality: Quality,
    /// Genre filter.
    pub genre: Genre,
    /// Rating filter.
    pub rating: Rating,
    /// Year filter.
    pub year: Year,
    /// Sorting order.
    pub order_by: OrderBy,
    /// Page number for pagination.
    pub page: u32,
}

impl Filter {
    /// Converts the quality filter to its string representation.
    pub fn quality_to_str(&self) -> &str {
        (&self.quality).into()
    }

    /// Converts the genre filter to its string representation.
    pub fn genre_to_str(&self) -> &str {
        (&self.genre).into()
    }

    /// Converts the rating filter to its string representation.
    pub fn rating_to_str(&self) -> &str {
        (&self.rating).into()
    }

    /// Converts the year filter to its string representation.
    ///
    /// Returns a `String` because some year variants represent ranges.
    pub fn year_to_str(&self) -> String {
        (&self.year).into()
    }

    /// Converts the order_by filter to its string representation.
    pub fn order_by_to_str(&self) -> &str {
        (&self.order_by).into()
    }
}

/// Represents video quality filter options.
#[derive(Debug)]
pub enum Quality {
    /// All qualities.
    All,
    /// 720p resolution.
    P720,
    /// 1080p resolution.
    P1080,
    /// 2160p (4K) resolution.
    P2160,
    /// 3D videos.
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
    /// Converts a string slice to a `Quality` variant.
    ///
    /// The conversion matches substrings "720", "1080", or "2160" to corresponding variants.
    /// Any other string defaults to `ThreeD`.
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

/// Represents rating filter options.
///
/// Ratings from 0 (All) to 9.
#[derive(Debug)]
pub enum Rating {
    All,
    /// Represents 1+
    One,
    /// Represents 2+
    Two,
    /// Represents 3+
    Three,
    /// Represents 4+
    Four,
    /// Represents 5+
    Five,
    /// Represents 6+
    Six,
    /// Represents 7+
    Seven,
    /// Represents 8+
    Eight,
    /// Represents 9+
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

/// Represents year filter options.
///
/// Includes specific years, ranges, or all years.
#[derive(Debug)]
pub enum Year {
    /// All years.
    All,
    /// Exact year.
    Equal(u32),
    /// Year range 2000-2009.
    Range2000to2009,
    /// Year range 1990-1999.
    Range1990to1999,
    /// Year range 1980-1989.
    Range1980to1989,
    /// Year range 1970-1979.
    Range1970to1979,
    /// Year range 1950-1969.
    Range1950to1969,
    /// Year range 1900-1949.
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

/// Represents ordering options for movie queries.
#[derive(Debug)]
pub enum OrderBy {
    /// Sort by latest.
    Latest,
    /// Sort by oldest.
    Oldest,
    /// Sort by featured.
    Featured,
    /// Sort by year.
    Year,
    /// Sort by rating.
    Rating,
    /// Sort by likes.
    Likes,
    /// Sort alphabetically.
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
