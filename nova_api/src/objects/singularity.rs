use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Singularity {
    pub error_feedback_name: String,
    pub singularity_type: Option<String>,
    pub singular_joint_position: Option<Vec<f64>>,
}
