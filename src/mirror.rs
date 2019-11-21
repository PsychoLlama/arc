use reqwest;
use semver::Version;
use serde::Deserialize;
use std::cmp::Ordering;
use std::env;

static DEFAULT_MIRROR: &str = "https://nodejs.org/dist";

#[derive(Deserialize, Debug)]
struct Listing {
    version: String,
    date: String,
}

#[derive(Debug, PartialEq)]
pub struct Distribution {
    pub version: Version,
    pub date: String,
}

fn get_node_mirror_url() -> String {
    env::var("ARC_NODEJS_MIRROR").unwrap_or_else(|_| DEFAULT_MIRROR.to_owned())
}

pub fn pull_distribution_list() -> Result<Vec<Distribution>, reqwest::Error> {
    let url = get_node_mirror_url() + "/index.json";

    let mut version_list: Vec<Distribution> = reqwest::get(&url)
        .or_else(describe_request_failure)?
        .json::<Vec<Listing>>()
        .or_else(describe_json_parse_failure)?
        .into_iter()
        .map(parse_listing)
        .collect();

    version_list.sort_by(order_by_version);

    Ok(version_list)
}

fn order_by_version(a: &Distribution, b: &Distribution) -> Ordering {
    a.version.cmp(&b.version)
}

fn parse_listing(listing: Listing) -> Distribution {
    let version = Version::parse(&listing.version[1..])
        .expect(format!("Couldn't parse node version string ({}).", &listing.version).as_ref());

    Distribution {
        version,
        date: listing.date,
    }
}

fn describe_request_failure(error: reqwest::Error) -> Result<reqwest::Response, reqwest::Error> {
    println!(
        "Couldn't fetch remote versions ({}):",
        get_node_mirror_url()
    );
    println!("{}", error);
    Err(error)
}

fn describe_json_parse_failure<Json>(error: reqwest::Error) -> Result<Json, reqwest::Error> {
    println!("Node.js mirror returned invalid JSON.");
    println!("{}", error);
    Err(error)
}
