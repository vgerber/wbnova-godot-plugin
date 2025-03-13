use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DhParameter {
    pub alpha: Option<f64>,
    pub reverse_rotation_direction: Option<bool>,
    pub theta: Option<f64>,
    pub a: Option<f64>,
    pub d: Option<f64>,
}
