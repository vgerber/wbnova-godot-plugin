use crate::objects::path::Path;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExecutionResult {
    pub motion_duration: f64,
    pub paths: Vec<Path>,
    pub motion_group_id: String,
}
