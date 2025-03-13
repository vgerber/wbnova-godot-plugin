use crate::objects::collision::Collision;
use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CollisionFeedback {
    pub error_feedback_name: String,
    pub collisions: Option<Vec<Collision>>,
    pub joint_position: Option<Vec<f64>>,
    pub tcp_pose: Option<Pose>,
}
