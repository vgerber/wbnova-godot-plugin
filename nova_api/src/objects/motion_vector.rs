use crate::objects::vector_3_d::Vector3D;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionVector {
    pub angular: Option<Vector3D>,
    pub coordinate_system: Option<String>,
    pub linear: Option<Vector3D>,
}
