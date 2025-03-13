use crate::objects::rotation_angles::RotationAngles;
use crate::objects::vector_3_d::Vector3D;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CoordinateSystem {
    pub name: Option<String>,
    pub coordinate_system: String,
    pub position: Option<Vector3D>,
    pub reference_uid: Option<String>,
    pub rotation: Option<RotationAngles>,
}
