use std::time::Duration;

use reqwest::header::USER_AGENT;

use super::default;
use crate::{Movie, Response, Torrent, client::Filter};

/// Client for interacting with the YTS movie API.
///
/// Provides methods to search for movies and retrieve torrent information.
///
/// # Examples
///
/// ```
/// # async fn example() -> crate::Result<()> {
/// let yts = blocking::Yts::default();
/// let response = yts.search("Inception")?;
/// for movie in response.movies {
///     println!("{:?}", movie);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct Yts<'a> {
    inner: default::Yts<'a>,
}

#[allow(dead_code)]
impl<'a> Yts<'a> {
    /// Creates a default `Yts` client with the official host and a 10-second timeout.
    pub fn new(host: &'a str, timeout: Duration) -> Self {
        Self {
            inner: default::Yts::new(host, timeout),
        }
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
    pub fn search_with_filter(&self, movie_name: &str, filter: Filter) -> crate::Result<Response> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(self.inner.create_url(movie_name, &filter))
            .header(USER_AGENT, "Mozilla/5.0 (Linux x86_64)")
            .timeout(self.inner.timeout)
            .send()?;

        Response::create(self.inner.host, &response.text()?, filter.page)
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
    pub fn search(&self, movie_name: &str) -> crate::Result<Response> {
        self.search_with_filter(movie_name, crate::Filters::default().build())
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
    pub fn torrents(&self, movie: &Movie) -> crate::Result<Vec<Torrent>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(&movie.link)
            .header(USER_AGENT, "Mozilla/5.0 (Linux x86_64)")
            .timeout(self.inner.timeout)
            .send()?;

        Torrent::create(self.inner.host, &response.text()?)
    }
}

#[cfg(test)]
mod test {
    use crate::Filters;

    use super::Yts;

    #[test]
    fn test_blocking_search() {
        let yts = Yts::default();
        let results = yts.search_with_filter(
            "the godfather",
            Filters::default().year(crate::Year::Equal(1974)).build(),
        );

        assert!(results.is_ok());
        assert!(!results.as_ref().unwrap().movies.is_empty());

        let torrents = yts.torrents(&results.unwrap().movies[0]);

        assert!(torrents.is_ok());
    }
}
