use crate::utils;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub pipelines: Pipelines,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub port: u32,
}

#[derive(Deserialize, Debug)]
pub struct Pipelines {
    pub dirs: Vec<String>,
}

pub fn load_config(filepath: &str) -> Config {
    let raw_file = utils::load_file(filepath);
    // TODO: Check that file parses correctly instead of unwrapping
    let config: Config = toml::from_str(&raw_file).unwrap();
    config
}
