use reqwest;
use serde::Deserialize;
use std::env;

static DEFAULT_MIRROR: &str = "https://nodejs.org/dist";

#[derive(Deserialize, Debug)]
struct Distribution {
    version: String,
    date: String,
}

pub fn ls() -> Result<(), reqwest::Error> {
    match fetch_version_list() {
        Ok(distributions) => display_version_list(distributions),
        Err(error) => println!("Error: {:?}", error),
    };

    Ok(())
}

fn display_version_list(distributions: Vec<Distribution>) {
    for dist in distributions {
        println!("* {}", dist.version);
    }
}

fn fetch_version_list() -> Result<Vec<Distribution>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = get_mirror() + "/index.json";

    let version_list: Vec<Distribution> = client.get(&url).send()?.json()?;

    Ok(version_list)
}

fn get_mirror() -> String {
    env::var("ARC_NODEJS_MIRROR").unwrap_or_else(|_| DEFAULT_MIRROR.to_owned())
}
