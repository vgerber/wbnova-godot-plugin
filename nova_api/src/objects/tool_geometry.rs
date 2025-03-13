use crate::objects::geometry::Geometry;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ToolGeometry {
    pub tcp: String,
    pub geometries: Option<Vec<Geometry>>,
}
