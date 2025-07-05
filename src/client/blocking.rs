use std::time::Duration;

use reqwest::header::USER_AGENT;

use super::default;
use crate::{Movie, Response, Torrent, client::Filter};

#[derive(Debug, Default)]
pub struct Yts<'a> {
    inner: default::Yts<'a>,
}

#[allow(dead_code)]
impl<'a> Yts<'a> {
    pub fn new(host: &'a str, timeout: Duration) -> Self {
        Self {
            inner: default::Yts::new(host, timeout),
        }
    }

    pub fn search_with_filter(&self, movie_name: &str, filter: Filter) -> crate::Result<Response> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(self.inner.create_url(movie_name, &filter))
            .header(USER_AGENT, "Mozilla/5.0 (Linux x86_64)")
            .timeout(self.inner.timeout)
            .send()?;

        Response::create(self.inner.host, &response.text()?, filter.page)
    }

    pub fn search(&self, movie_name: &str) -> crate::Result<Response> {
        self.search_with_filter(movie_name, crate::Filters::default().build())
    }

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

        let torrents = yts.torrents(&results.unwrap().movies[0]);
        println!("{torrents:#?}");
    }
}
