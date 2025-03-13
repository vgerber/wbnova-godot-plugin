use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SingleJointLimit {
    pub joint: String,
    pub limit: f64,
}
