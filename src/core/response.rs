use scraper::{Html, Selector};

use crate::{Genre, Quality};

use super::model;

/// Represents pagination information for a movie list page.
#[derive(Debug)]
pub struct Page {
    /// The current page number.
    pub current: u32,
    /// The total number of pages.
    pub of: u32,
    /// The total number of movies available.
    pub total: u32,
}

impl Page {
    /// Creates a new `Page` instance calculating total pages based on total items.
    ///
    /// # Parameters
    /// - `current`: The current page number.
    /// - `total`: The total number of movies available.
    ///
    /// # Returns
    /// A `Page` struct with calculated total pages (`of`).
    ///
    /// # Notes
    /// Assumes 20 movies per page.
    fn create(current: u32, total: u32) -> Self {
        let of = if total > 20 {
            (total / 20) + (if total % 20 > 0 { 1 } else { 0 })
        } else {
            1
        };

        Self { current, of, total }
    }
}

/// Represents the response from a movie listing page.
///
/// Contains pagination info and a list of parsed movies.
#[derive(Debug)]
pub struct Response {
    /// Pagination information.
    pub page: Page,
    /// List of movies parsed from the page.
    pub movies: Vec<model::Movie>,
}

impl Response {
    /// Parses HTML content to create a `Response` with movies and pagination info.
    ///
    /// # Parameters
    /// - `host`: Base URL host to prefix relative links.
    /// - `html`: Raw HTML content of the page.
    /// - `page`: Current page number.
    ///
    /// # Returns
    /// A `Result` containing the parsed `Response` or an error.
    ///
    /// # Errors
    /// Returns errors if parsing fails or required movie data is missing.
    pub(crate) fn create(host: &str, html: &str, page: u32) -> crate::Result<Self> {
        let document = Html::parse_document(html);

        let total: u32 = document
            .select(&Selector::parse("div.container h2 b")?)
            .next()
            .and_then(|value| value.text().next())
            .and_then(|value| value.parse().ok())
            .unwrap_or_default();

        let mut movies = Vec::new();
        if let Some(div) = document.select(&Selector::parse("section div.row")?).next() {
            for line in div.select(&Selector::parse("div.browse-movie-wrap")?) {
                let link = format!(
                    "{}{}",
                    host,
                    line.select(&Selector::parse("a.browse-movie-link")?)
                        .next()
                        .and_then(|e| e.attr("href"))
                        .unwrap_or_default()
                );

                let image = format!(
                    "{}{}",
                    host,
                    line.select(&Selector::parse("img")?)
                        .next()
                        .and_then(|e| e.attr("src"))
                        .unwrap_or_default()
                );

                let info = line
                    .text()
                    .filter(|&t| !t.trim().is_empty() && t != "View Details")
                    .collect::<Vec<_>>();

                let rating = info.first().ok_or(crate::Error::MovieRatingError)?;
                let rating = &rating[..2];
                let rating: f32 = rating.parse()?;

                let year: u32 = info.last().ok_or(crate::Error::MovieYearError)?.parse()?;

                let name = info
                    .get(info.len() - 2)
                    .ok_or(crate::Error::MovieNameError)?
                    .to_string();

                let mut genres = Vec::new();
                for &value in &info[1..info.len() - 2] {
                    let value: Genre = value.into();
                    genres.push(value);
                }

                movies.push(model::Movie::new(name, year, rating, genres, image, link));
            }
        }

        Ok(Self {
            page: Page::create(page, total),
            movies,
        })
    }
}

/// Represents a torrent download option for a movie.
#[derive(Debug)]
pub struct Torrent {
    /// The quality of the torrent (e.g., 720p, 1080p).
    pub quality: Quality,
    /// The size of the torrent file.
    pub size: String,
    /// The language of the torrent.
    pub language: String,
    /// The runtime of the movie.
    pub runtime: String,
    /// Information about peers and seeds.
    pub peers_seeds: String,
    /// Direct link to the torrent file.
    pub link: String,
}

impl Torrent {
    /// Creates a new `Torrent` instance from raw string data.
    ///
    /// # Parameters
    /// - `quality`: Quality string (converted to `Quality` enum).
    /// - `size`: Size of the torrent.
    /// - `language`: Language of the torrent.
    /// - `runtime`: Runtime of the movie.
    /// - `peers_seeds`: Peers and seeds info.
    /// - `link`: URL link to the torrent.
    ///
    /// # Returns
    /// A new `Torrent` struct.
    pub(crate) fn new(
        quality: &str,
        size: &str,
        language: &str,
        runtime: &str,
        peers_seeds: &str,
        link: String,
    ) -> Self {
        Self {
            quality: quality.into(),
            size: size.to_string(),
            language: language.to_string(),
            runtime: runtime.to_string(),
            peers_seeds: peers_seeds.to_string(),
            link,
        }
    }

    /// Parses HTML content to extract a list of torrents.
    ///
    /// # Parameters
    /// - `host`: Base URL host to prefix relative links.
    /// - `html`: Raw HTML content containing torrent info.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Torrent` structs or an error.
    pub(crate) fn create(host: &str, html: &str) -> crate::Result<Vec<Self>> {
        let document = Html::parse_document(html);

        let mut torrents = Vec::new();
        if let Some(movie_tech_specs) = document
            .select(&Selector::parse("div#movie-tech-specs")?)
            .next()
        {
            let qualities = movie_tech_specs
                .select(&Selector::parse("span.tech-quality")?)
                .map(|line| line.text().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let data = movie_tech_specs
                .select(&Selector::parse("div.tech-spec-info")?)
                .map(|line| {
                    line.text()
                        .map(|t| t.trim())
                        .filter(|&t| Self::skip_useless_str(t))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            if let Some(movie_info) = document
                .select(&Selector::parse("div#movie-info p")?)
                .next()
            {
                for (i, line) in movie_info.select(&Selector::parse("a")?).enumerate() {
                    let link = format!("{}{}", host, line.attr("href").unwrap_or_default());

                    let data = &data[i];
                    let qualities = &qualities[i];

                    torrents.push(Torrent::new(
                        qualities[0],
                        data[0],
                        data[2],
                        data[3],
                        data[4],
                        link,
                    ));
                }
            }
        }

        Ok(torrents)
    }

    /// Helper function to filter out unwanted strings during parsing.
    ///
    /// Returns `true` if the string is useful, `false` if it should be ignored.
    fn skip_useless_str(value: &str) -> bool {
        !value.is_empty()
            && value != "P/S"
            && value != "Subtitles"
            && value != "NR"
            && !value.contains("fps")
    }
}
