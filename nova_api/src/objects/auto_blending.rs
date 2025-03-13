use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AutoBlending {
    pub min_velocity_in_percent: Option<i32>,
    pub blending_name: String,
}
