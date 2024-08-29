pub mod muni_geo;
pub mod votes_lpz;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    vote_sources: Vec<VoteSource>,
}

#[derive(Deserialize, Debug)]
struct VoteSource {
    url: String,
    name: String,
    party_map: HashMap<String, String>,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("harvester.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
