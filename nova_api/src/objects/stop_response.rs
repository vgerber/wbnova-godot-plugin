use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StopResponse {
    pub location_on_trajectory: f64,
    pub stop_code: String,
    pub message: Option<String>,
}
