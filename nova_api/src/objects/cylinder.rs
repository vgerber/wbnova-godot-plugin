use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Cylinder {
    pub height: f64,
    pub radius: f64,
}
