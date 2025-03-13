use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CollisionContact {
    pub world: Option<Vec<f64>>,
    pub local: Option<Vec<f64>>,
}
