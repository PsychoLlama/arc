use serde::Deserialize;

use super::mirror::{pull_distribution_list, Distribution};

#[derive(Deserialize, Debug)]
struct Listing {
    version: String,
    date: String,
}

pub fn ls() {
    match pull_distribution_list() {
        Ok(distributions) => display_version_list(distributions),
        Err(_) => std::process::exit(1),
    };
}

fn display_version_list(distributions: Vec<Distribution>) {
    for dist in distributions {
        println!("* {}", dist.version);
    }
}
