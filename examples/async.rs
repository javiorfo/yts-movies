use yts_movies::{Filters, OrderBy, Year, Yts};

#[tokio::main]
async fn main() {
    let yts = Yts::default();
    let response = yts
        .search_with_filter(
            "the godfather",
            Filters::default()
                .year(Year::Range1970to1979)
                .order_by(OrderBy::Rating)
                .build(),
        )
        .await
        .expect("Error searching movies");

    println!("{response:#?}");

    // Getting the torrents of the first movie
    let torrents = yts
        .torrents(&response.movies[0])
        .await
        .expect("error searching torrents");

    println!("{torrents:#?}");
}
