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
    pub scheduled_time: String,
    pub execution_start: String,
    pub execution_end: String,
    pub status: String,
    pub logs: String,
    pub created_at: String,
}
