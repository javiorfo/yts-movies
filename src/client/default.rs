use std::time::Duration;

use reqwest::{Url, header::USER_AGENT};

use crate::{Movie, Response, Torrent, client::Filter};

/// Client for interacting with the YTS movie API.
///
/// Provides methods to search for movies and retrieve torrent information.
///
/// # Examples
///
/// ```
/// use yts_movies;
///
/// # async fn example() -> yts_movies::Result {
///     let yts = yts_movies::Yts::default();
///     let response = yts.search("Inception").await?;
///     for movie in response.movies {
///         println!("{:?}", movie);
///     }
/// #   Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Yts<'a> {
    /// Base URL of the YTS API host.
    pub(crate) host: &'a str,
    /// Request timeout duration.
    pub(crate) timeout: Duration,
}

impl Default for Yts<'_> {
    /// Creates a default `Yts` client with the official host and a 10-second timeout.
    fn default() -> Self {
        Self {
            host: "https://en.yts-official.mx",
            timeout: Duration::from_secs(10),
        }
    }
}

impl<'a> Yts<'a> {
    /// Creates a new `Yts` client with a custom host and timeout.
    ///
    /// # Parameters
    /// - `host`: Base URL of the YTS API.
    /// - `timeout`: Duration before requests time out.
    ///
    /// # Returns
    /// A new instance of `Yts`.
    pub fn new(host: &'a str, timeout: Duration) -> Self {
        Self { host, timeout }
    }

    /// Searches for movies by name applying the specified filter options.
    ///
    /// # Parameters
    /// - `movie_name`: The name or keyword to search for.
    /// - `filter`: A `Filter` struct specifying search filters (quality, genre, etc.).
    ///
    /// # Returns
    /// A `Result` containing a `Response` with movies and pagination info, or an error.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or the response cannot be parsed.
    pub async fn search_with_filter(
        &self,
        movie_name: &str,
        filter: Filter,
    ) -> crate::Result<Response> {
        let client = reqwest::Client::new();

        let response = client
            .get(self.create_url(movie_name, &filter)?)
            .header(USER_AGENT, "Mozilla/5.0 (Linux x86_64)")
            .timeout(self.timeout)
            .send()
            .await?;

        Response::create(self.host, &response.text().await?, filter.page)
    }

    /// Searches for movies by name using default filter parameters.
    ///
    /// # Parameters
    /// - `movie_name`: The name or keyword to search for.
    ///
    /// # Returns
    /// A `Result` containing a `Response` with movies and pagination info, or an error.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or the response cannot be parsed.
    pub async fn search(&self, movie_name: &str) -> crate::Result<Response> {
        self.search_with_filter(movie_name, crate::Filters::default().build())
            .await
    }

    /// Retrieves torrent information for a given movie.
    ///
    /// # Parameters
    /// - `movie`: Reference to a `Movie` struct.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Torrent` structs or an error.
    ///
    /// # Errors
    /// Returns an error if the HTTP request fails or the response cannot be parsed.
    pub async fn torrents(&self, movie: &Movie) -> crate::Result<Vec<Torrent>> {
        let client = reqwest::Client::new();

        let response = client
            .get(&movie.link)
            .header(USER_AGENT, "Mozilla/5.0 (Linux x86_64)")
            .timeout(self.timeout)
            .send()
            .await?;

        Torrent::create(self.host, &response.text().await?)
    }

    /// Constructs the URL for a movie search with the specified filters.
    ///
    /// # Parameters
    /// - `movie_name`: The movie name or keyword to search for.
    /// - `filter`: Reference to a `Filter` struct containing filter parameters.
    ///
    /// # Returns
    /// A `String` containing the fully constructed URL.
    pub(crate) fn create_url(&self, movie_name: &str, filter: &Filter) -> crate::Result<String> {
        let mut url: reqwest::Url = Url::parse(&format!("{}/browse-movies", self.host))
            .map_err(|_| crate::Error::ParseError(self.host.to_string()))?;

        url.query_pairs_mut()
            .append_pair("keyword", movie_name.trim());

        Ok(format!(
            "{}&quality={}&genre={}&rating={}&year={}&order_by={}&page={}",
            url.as_str(),
            filter.quality_to_str(),
            filter.genre_to_str(),
            filter.rating_to_str(),
            filter.year_to_str(),
            filter.order_by_to_str(),
            filter.page
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::Filters;

    use super::Yts;

    #[tokio::test]
    async fn test_async_search_with_filters() {
        let yts = Yts::default();
        let results = yts
            .search_with_filter(
                "the godfather",
                Filters::default().year(crate::Year::Equal(1974)).build(),
            )
            .await;

        assert!(results.is_ok());
        assert!(!results.as_ref().unwrap().movies.is_empty());

        let torrents = yts.torrents(&results.unwrap().movies[0]).await;

        assert!(torrents.is_ok());
    }

    #[tokio::test]
    async fn test_async_search() {
        let yts = Yts::default();
        let results = yts.search("killers of the flower moon").await;

        assert!(results.is_ok());
        assert!(!results.as_ref().unwrap().movies.is_empty());

        let torrents = yts.torrents(&results.unwrap().movies[0]).await;

        assert!(torrents.is_ok());
    }
}
