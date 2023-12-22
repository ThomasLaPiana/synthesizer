use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Task {
    pub id: String,
    pub pipeline_id: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Pipeline {
    pub id: String,
    pub schedule: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct TaskInstance {
    pub id: String,
    pub task_id: String,
    pub pipeline_id: String,
    pub execution_time: String,
    pub status: String,
    pub logs: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Manifest {
    pub pipelines: Vec<Pipeline>,
    pub tasks: Vec<Task>,
}
