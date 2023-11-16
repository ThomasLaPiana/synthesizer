use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Task {
    pub name: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub schedule: String,
}
