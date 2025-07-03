use reqwest::blocking::get;

use crate::Response;

#[allow(dead_code)]
pub fn search() -> crate::Result<Response> {
    let response = get(
        "https://en.yts-official.mx/browse-movies?keyword=clean&quality=all&genre=all&rating=0&year=0&order_by=latest",
    )?;

    let response = Response::create("https://en.yts-official.mx", &response.text()?, 1)?;

    Ok(response)
}

#[cfg(test)]
mod test {
    use super::search;

    #[test]
    fn test_blocking_search() {
        let results = search();
        println!("{results:#?}");
    }
}
