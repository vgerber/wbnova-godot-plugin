use crate::objects::quaternion::Quaternion;
use crate::objects::vector_3_d::Vector3D;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlannerPose {
    pub orientation: Option<Quaternion>,
    pub position: Option<Vector3D>,
}
