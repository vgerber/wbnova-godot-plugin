use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GenericBox {
    pub size_x: f64,
    pub size_z: f64,
    pub size_y: f64,
    #[serde(alias = "type")]
    pub type_name: String,
}
