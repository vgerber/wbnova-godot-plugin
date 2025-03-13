use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExternalJointStreamDatapointValue {
    pub velocities: Vec<f64>,
    pub accelerations: Vec<f64>,
    pub positions: Vec<f64>,
    pub torques: Vec<f64>,
}
