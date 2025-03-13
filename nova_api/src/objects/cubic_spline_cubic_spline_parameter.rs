use crate::objects::pose::Pose;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CubicSplineCubicSplineParameter {
    pub pose: Pose,
    pub path_parameter: f64,
}
