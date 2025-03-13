use crate::objects::vector_3_d::Vector3D;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DirectionJoggingRequest {
    pub rotation_velocity: f64,
    pub response_coordinate_system: Option<String>,
    pub motion_group: String,
    pub coordinate_system: Option<String>,
    pub position_direction: Vector3D,
    pub position_velocity: f64,
    pub response_rate: Option<i32>,
    pub rotation_direction: Vector3D,
}
