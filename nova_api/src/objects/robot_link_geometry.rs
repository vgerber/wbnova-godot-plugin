use crate::objects::geometry::Geometry;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RobotLinkGeometry {
    pub geometry: Geometry,
    pub link_index: i32,
}
