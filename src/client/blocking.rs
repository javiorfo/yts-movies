use std::time::Duration;

use crate::{Response, client::Filter};

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
            .timeout(self.timeout)
            .send()?;

        let response = Response::create(self.host, &response.text()?, filter.page)?;

        Ok(response)
    }

    pub fn search(&self, movie_name: &str) -> crate::Result<Response> {
        self.search_with_filter(movie_name, crate::Filters::default().build())
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
    use super::Yts;
    use crate::Filters;

    #[test]
    fn test_blocking_search() {
        let client = Yts::default();
        let results = client.search_with_filter("caca", Filters::default().page(1).build());
        println!("{results:#?}");
    }
}
