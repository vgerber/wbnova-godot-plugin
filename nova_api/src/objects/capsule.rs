use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Capsule {
    pub cylinder_height: f64,
    pub radius: f64,
}
