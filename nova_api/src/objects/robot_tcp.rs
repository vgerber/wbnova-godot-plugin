use crate::objects::rotation_angles::RotationAngles;
use crate::objects::vector_3_d::Vector3D;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RobotTcp {
    pub id: String,
    pub readable_name: Option<String>,
    pub rotation: Option<RotationAngles>,
    pub position: Vector3D,
}
