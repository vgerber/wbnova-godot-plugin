use crate::objects::vector_3_d::Vector3D;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TcpPose {
    pub orientation: Vector3D,
    pub coordinate_system: Option<String>,
    pub position: Vector3D,
    pub tcp: String,
}
