use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RotationAngles {
    pub angles: Vec<f64>,
    #[serde(alias = "type")]
    pub type_name: String,
}
