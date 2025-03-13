use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JointLimitExceeded {
    pub joint_position: Option<Vec<f64>>,
    pub error_feedback_name: String,
    pub joint_index: Option<i32>,
}
