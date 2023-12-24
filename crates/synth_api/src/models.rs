use serde::{Deserialize, Serialize};
use synth_common::models::Task;

/// Generic Struct used for API Responses
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct JSONResponse<T> {
    pub data: Option<Vec<T>>,
    pub errors: Option<Vec<String>>,
}

pub type TaskJSONResponse = JSONResponse<Task>;
