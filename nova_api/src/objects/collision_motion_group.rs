use crate::objects::collision_motion_group_link::CollisionMotionGroupLink;
use crate::objects::collision_motion_group_tool::CollisionMotionGroupTool;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CollisionMotionGroup {
    pub link_chain: Option<Vec<CollisionMotionGroupLink>>,
    pub tool: Option<CollisionMotionGroupTool>,
}
