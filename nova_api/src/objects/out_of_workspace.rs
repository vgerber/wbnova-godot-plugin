use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OutOfWorkspace {
    pub invalid_tcp_pose: Option<Pose>,
    pub error_feedback_name: String,
}
