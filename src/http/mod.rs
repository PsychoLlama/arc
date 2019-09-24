use reqwest;
use scraper::{Html, Selector};

pub fn fetch_version_list(mirror: &str) -> Result<Vec<String>, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(mirror).send()?.text()?;

    let versions = scrape_available_version_list(response);

    Ok(versions)
}

fn scrape_available_version_list(html: String) -> Vec<String> {
    let document = Html::parse_document(&html);
    let selector = Selector::parse("a").unwrap();
    let links = document.select(&selector);

    links
        .into_iter()
        .map(|link| link.value().attr("href"))
        .filter(|href| href.is_some())
        .map(|href| href.unwrap().to_owned())
        .collect()
}
