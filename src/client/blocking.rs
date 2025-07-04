use std::time::Duration;

use reqwest::header::USER_AGENT;

use crate::{Movie, Response, Torrent, client::Filter};

pub struct Yts<'a> {
    host: &'a str,
    timeout: Duration,
}

impl Default for Yts<'_> {
    fn default() -> Self {
        Self {
            host: "https://en.yts-official.mx",
            timeout: Duration::from_secs(10),
        }
    }
}

impl<'a> Yts<'a> {
    pub fn new(host: &'a str, timeout: Duration) -> Self {
        Self { host, timeout }
    }

    pub fn search_with_filter(&self, movie_name: &str, filter: Filter) -> crate::Result<Response> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(self.create_url(movie_name, &filter))
            .header(USER_AGENT, "Mozilla/5.0 (Linux x86_64)")
            .timeout(self.timeout)
            .send()?;

        Response::create(self.host, &response.text()?, filter.page)
    }

    pub fn search(&self, movie_name: &str) -> crate::Result<Response> {
        self.search_with_filter(movie_name, crate::Filters::default().build())
    }

    pub fn torrents(&self, movie: &Movie) -> crate::Result<Vec<Torrent>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .get(&movie.link)
            .header(USER_AGENT, "Mozilla/5.0 (Linux x86_64)")
            .timeout(self.timeout)
            .send()?;

        Torrent::create(self.host, &response.text()?)
    }

    fn create_url(&self, movie_name: &str, filter: &Filter) -> String {
        let movie_name = movie_name.trim().replace(" ", "+");

        format!(
            "{}/browse-movies?keyword={}&quality={}&genre={}&rating={}&year={}&order_by={}&page={}",
            self.host,
            movie_name,
            filter.quality_to_str(),
            filter.genre_to_str(),
            filter.rating_to_str(),
            filter.year_to_str(),
            filter.order_by_to_str(),
            filter.page
        )
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
