use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JointLimit {
    pub unlimited: Option<bool>,
    pub lower_limit: f64,
    pub upper_limit: f64,
    pub joint: String,
}
