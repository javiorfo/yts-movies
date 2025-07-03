use scraper::{Html, Selector};

use super::model;

#[derive(Debug)]
pub struct Page {
    pub current: u32,
    pub of: u32,
    pub total: u32,
}

impl Page {
    fn create(current: u32, total: u32) -> Self {
        let of = if total > 20 {
            (total / 20) + (if total % 20 > 0 { 1 } else { 0 })
        } else {
            1
        };

        Self { current, of, total }
    }
}

#[derive(Debug)]
pub struct Response {
    page: Page,
    movies: Vec<model::Movie>,
}

impl Response {
    pub(crate) fn create(host: &str, html: &str, page: u32) -> crate::Result<Self> {
        let document = Html::parse_document(html);

        let total: u32 = document
            .select(&Selector::parse("div.container h2 b")?)
            .next()
            .ok_or(crate::Error::MovieCountError)?
            .text()
            .next()
            .and_then(|value| value.parse().ok())
            .ok_or(crate::Error::MovieCountError)?;

        let mut movies = Vec::new();
        if let Some(table) = document.select(&Selector::parse("section div.row")?).next() {
            for line in table.select(&Selector::parse("div.browse-movie-wrap")?) {
                let link = format!(
                    "{}{}",
                    host,
                    line.select(&Selector::parse("a.browse-movie-link")?)
                        .next()
                        .and_then(|e| e.attr("href"))
                        .ok_or(crate::Error::MovieLinkError)?
                );

                let image = format!(
                    "{}{}",
                    host,
                    line.select(&Selector::parse("img")?)
                        .next()
                        .and_then(|e| e.attr("src"))
                        .ok_or(crate::Error::MovieImageError)?
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

                movies.push(model::Movie::new(name, year, rating, vec![], image, link));
            }
        }

        Ok(Self {
            page: Page::create(page, total),
            movies,
        })
    }
}
