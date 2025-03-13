use crate::objects::vector_3_d::Vector3D;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Payload {
    pub moment_of_inertia: Option<Vector3D>,
    pub name: String,
    pub center_of_mass: Option<Vector3D>,
    pub payload: f64,
}
