use crate::objects::collider_dictionary::ColliderDictionary;
use crate::objects::collision_motion_group::CollisionMotionGroup;
use crate::objects::optimizer_setup::OptimizerSetup;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlanTrajectoryRequest {
    pub start_joint_position: Vec<f64>,
    pub collision_motion_group: Option<CollisionMotionGroup>,
    pub robot_setup: OptimizerSetup,
    pub static_colliders: Option<ColliderDictionary>,
}
