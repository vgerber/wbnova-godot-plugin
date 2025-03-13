use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RectangularCapsule {
    pub radius: f64,
    pub sphere_center_distance_x: f64,
    pub sphere_center_distance_y: f64,
}
