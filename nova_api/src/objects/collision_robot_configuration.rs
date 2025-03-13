use crate::objects::object::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CollisionRobotConfiguration {
    pub link_attachements: Option<Object>,
    pub tool: Option<Object>,
    pub use_default_link_shapes: bool,
}
