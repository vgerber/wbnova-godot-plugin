use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Collider {
    pub margin: Option<f64>,
    pub pose: Option<Pose>,
}
