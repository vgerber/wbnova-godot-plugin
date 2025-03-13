use crate::objects::collision_motion_group_link::CollisionMotionGroupLink;
use crate::objects::collision_motion_group_tool::CollisionMotionGroupTool;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CollisionMotionGroupAssembly {
    pub stored_tool: Option<String>,
    pub stored_link_chain: Option<String>,
    pub tool: Option<CollisionMotionGroupTool>,
    pub link_chain: Option<Vec<CollisionMotionGroupLink>>,
}
