use crate::objects::geometry::Geometry;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SafetyZone {
    pub geometry: Geometry,
    pub id: i32,
    pub priority: i32,
}
