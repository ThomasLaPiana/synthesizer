use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Task {
    pub name: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Pipeline {
    pub name: String,
    pub schedule: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Pipelines {
    pub pipelines: Vec<Pipeline>,
}
