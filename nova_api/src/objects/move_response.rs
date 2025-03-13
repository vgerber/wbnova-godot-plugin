use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MoveResponse {
    pub current_location_on_trajectory: Option<f64>,
    pub time_to_end: Option<i32>,
}
