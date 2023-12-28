use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Manifest {
    pub pipelines: Vec<ManifestPipeline>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ManifestPipeline {
    pub id: String,
    pub schedule: String,
    pub tasks: Vec<ManifestTask>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ManifestTask {
    pub id: String,
    pub command: String,
}
