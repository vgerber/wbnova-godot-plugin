use crate::objects::geometry::Geometry;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SafetySetupSafetyZone {
    pub id: Option<i32>,
    pub priority: Option<i32>,
    pub motion_group_uid: Option<i32>,
    pub geometry: Option<Geometry>,
}
