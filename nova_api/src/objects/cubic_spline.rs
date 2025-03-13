use crate::objects::cubic_spline_cubic_spline_parameter::CubicSplineCubicSplineParameter;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CubicSpline {
    pub parameters: Vec<CubicSplineCubicSplineParameter>,
}
