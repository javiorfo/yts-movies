use scraper::{Html, Selector};

use crate::{Genre, Quality};

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
    pub page: Page,
    pub movies: Vec<model::Movie>,
}

impl Response {
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

#[derive(Debug)]
pub struct Torrent {
    pub quality: Quality,
    pub size: String,
    pub language: String,
    pub runtime: String,
    pub peers_seeds: String,
    pub link: String,
}

impl Torrent {
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
                        .filter(|&t| Self::remove_useless_str(t))
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

    fn remove_useless_str(value: &str) -> bool {
        !value.is_empty()
            && value != "P/S"
            && value != "Subtitles"
            && value != "NR"
            && !value.contains("fps")
    }
}
