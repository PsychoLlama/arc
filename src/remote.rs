use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Distribution {
    version: String,
    date: String,
}

pub fn ls(mirror: &str) -> Result<(), reqwest::Error> {
    for dist in fetch_version_list(&mirror)? {
        println!("* {}", dist.version);
    }

    Ok(())
}

fn fetch_version_list(mirror: &str) -> Result<Vec<Distribution>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = String::from(mirror) + "/index.json";
    let version_list: Vec<Distribution> = client.get(&url).send()?.json()?;

    Ok(version_list)
}
