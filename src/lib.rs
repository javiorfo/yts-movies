mod client;
mod core;

pub use client::{Filters, OrderBy, Quality, Rating, Year};
pub use core::{
    Page, Response, Torrent,
    model::{Genre, Movie},
};

#[cfg(feature = "async")]
pub use client::default::search;

#[cfg(feature = "blocking")]
pub use client::blocking;

/// Error type for all fallible operations in this crate.
///
/// Wraps errors from underlying dependencies such as [`reqwest`] and [`scraper`].
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    ToStrError(#[from] reqwest::header::ToStrError),

    #[error(transparent)]
    SelectorError(#[from] scraper::error::SelectorErrorKind<'static>),

    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Error getting movie rating")]
    MovieRatingError,

    #[error("Error getting movie year")]
    MovieYearError,

    #[error("Error getting movie name")]
    MovieNameError,
}

/// Convenient result type for this crate.
pub type Result<T = ()> = std::result::Result<T, Error>;
