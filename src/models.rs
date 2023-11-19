use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Task {
    pub name: String,
    pub pipeline_id: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Pipeline {
    pub id: String,
    pub name: Option<String>,
    pub schedule: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Manifest {
    pub pipelines: Vec<Pipeline>,
    pub tasks: Vec<Task>,
}
