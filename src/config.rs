use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub server: ServerSettings,
    pub pipelines: PipelineSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct PipelineSettings {
    pub dirs: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ServerSettings {
    pub scheme: String,
    pub host: String,
    pub port: u32,
}
pub trait BuildUrl {
    fn build_url(&self) -> String;
}
impl BuildUrl for ServerSettings {
    fn build_url(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port)
    }
}

pub fn load_config(filepath: &str) -> Result<Settings, config::ConfigError> {
    Config::builder()
        .add_source(File::with_name(filepath))
        .add_source(Environment::default().prefix("SYNTH"))
        .build()?
        .try_deserialize::<Settings>()
}
