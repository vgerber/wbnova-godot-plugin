use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub size_y: f64,
    pub size_x: f64,
}
