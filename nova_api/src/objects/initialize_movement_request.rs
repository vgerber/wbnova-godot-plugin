use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct InitializeMovementRequest {
    pub message_type: Option<String>,
    pub trajectory: String,
    pub response_coordinate_system: Option<String>,
    pub initial_location: Option<f64>,
    pub response_rate: Option<i32>,
}
