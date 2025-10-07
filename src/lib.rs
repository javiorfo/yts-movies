//! # YTS Movie API Client
//!
//! This crate provides an interface for searching and retrieving movie information from the YTS API,
//! including filtering options, pagination, and torrent details.
//!
//! ## Features
//! - Async and blocking HTTP clients (enabled via feature flags `async` and `blocking`).
//! - Rich filtering options such as quality, genre, rating, year, and sorting order.
//! - Parsing of HTML responses to extract movie and torrent metadata.
//!
//!
//! ### Async Example (default)
//!
//! ```
//! # #[cfg(feature = "async")]
//! use yts_movies::{Filters, OrderBy, Year, Yts};
//!
//! #[tokio::main]
//! async fn main() -> yts_movies::Result {
//!     let yts = Yts::default();
//!     let response = yts
//!         .search_with_filter(
//!             "the godfather",
//!             Filters::default()
//!                 .year(Year::Range1970to1979)
//!                 .order_by(OrderBy::Rating)
//!                 .build(),
//!         )
//!         .await?;
//!
//!     println!("{response:#?}");
//!
//!     // Getting the torrents of the first movie
//!     let torrents = yts
//!         .torrents(&response.movies[0])
//!         .await?;
//!
//!     println!("{torrents:#?}");
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Modules & Re-exports
//!
//! The crate re-exports key types for convenience:
//! - Filtering options: [`Filters`], [`OrderBy`], [`Quality`], [`Rating`], [`Year`]
//! - Core types: [`Page`], [`Response`], [`Torrent`], [`Genre`], [`Movie`]
//! - Client structs: [`Yts`] (async) and blocking client (behind feature flags)
//!
//! ## Error Handling
//!
//! All fallible operations return [`Result<T>`](Result) with a custom [`Error`] enum that wraps
//! errors from underlying dependencies (e.g., `reqwest`, `scraper`).
//!
//! ## Feature Flags
//!
//! - `async` — Enables the asynchronous API (`search`).
//! - `blocking` — Enables the blocking (synchronous) API (`blocking::search`).
//!
//! ## License
//!
//! This is free software, published under the [MIT License](https://mit-license.org/).
//!
//! ## See Also
//!
//! - [`reqwest`] — HTTP client for requests.
//! - [`scraper`] — HTML parsing for subtitle extraction.

mod client;
mod core;

pub use client::{Filters, OrderBy, Quality, Rating, Year};
pub use core::{
    Page, Response, Torrent,
    model::{Genre, Movie},
};

#[cfg(feature = "async")]
pub use client::default::Yts;

#[cfg(feature = "blocking")]
pub use client::blocking;

/// Errors that can occur when using this crate.
///
/// This enum wraps errors from underlying dependencies such as [`reqwest`] and [`scraper`],
/// as well as custom errors related to parsing movie data.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error originating from an HTTP request failure.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// Error converting HTTP header values to strings.
    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError),

    /// Error parsing CSS selectors during HTML scraping.
    #[error(transparent)]
    SelectorError(#[from] scraper::error::SelectorErrorKind<'static>),

    /// Error parsing a floating point number.
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    /// Error parsing an integer.
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    /// Error indicating failure to extract the movie rating from HTML.
    #[error("Error getting movie rating")]
    MovieRatingError,

    /// Error indicating failure to extract the movie year from HTML.
    #[error("Error getting movie year")]
    MovieYearError,

    /// Error indicating failure to extract the movie name from HTML.
    #[error("Error getting movie name")]
    MovieNameError,

    /// Error parsing an url.
    #[error("Error parsing url {0}")]
    ParseError(String),
}

/// A convenient alias for `Result` with the crate's [`Error`] type.
///
/// Defaults to `()` for the success type if not specified.
pub type Result<T = ()> = std::result::Result<T, Error>;
