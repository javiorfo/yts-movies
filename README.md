# yts-movies
*Library to search YTS movies info and torrent links*

## Description
This crate provides an interface for searching and retrieving movie information from the YTS API, including filtering options, pagination, and torrent details.
It offers both asynchronous and blocking (synchronous) interfaces, with flexible filtering and ordering options. 
**It uses a web scraper to build the api** 

## Usage
Add this crate to your `Cargo.toml`:

```toml
[dependencies]
yts-movies = "0.1.0"
```

#### Enable blocking feature if needed

```toml
[dependencies]
yts-movies = { version = "0.1.0", features = ["blocking"] }
```

## Async Example (default)

```rust
use yts_movies::{Filters, OrderBy, Year, Yts};

#[tokio::main]
async fn main() -> yts_movies::Result {
    let yts = Yts::default();
    let response = yts
        .search_with_filter(
            "the godfather",
            Filters::default()
                .year(Year::Range1970to1979)
                .order_by(OrderBy::Rating)
                .build(),
        )
        .await?;

    println!("{response:#?}");

    // Getting the torrents of the first movie
    let torrents = yts
        .torrents(&response.movies[0])
        .await?;

    println!("{torrents:#?}");

    Ok(())
}
```

## Details
- Here are more [examples](https://github.com/javiorfo/yts-movies/tree/master/examples)

## Features
- Default async search. Blocking search available too
- Search by movie name and/or filters (quality, genre, rating, page, ordering and year)
- Obtain not only info and metadata but also a torrent download link of the movie.

## Docs
Find all the configuration options in the full [documentation](https://docs.rs/yts-movies/0.1.0/yts_movies/).

---

### Donate
- **Bitcoin** [(QR)](https://raw.githubusercontent.com/javiorfo/img/master/crypto/bitcoin.png)  `1GqdJ63RDPE4eJKujHi166FAyigvHu5R7v`
- [Paypal](https://www.paypal.com/donate/?hosted_button_id=FA7SGLSCT2H8G)
