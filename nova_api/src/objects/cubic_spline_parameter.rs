use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CubicSplineParameter {
    pub pose: Pose,
    pub path_parameter: f64,
}
