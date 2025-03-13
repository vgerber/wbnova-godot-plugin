use crate::objects::vector_3_d::Vector3D;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ForceVector {
    pub coordinate_system: Option<String>,
    pub moment: Option<Vector3D>,
    pub force: Option<Vector3D>,
}
