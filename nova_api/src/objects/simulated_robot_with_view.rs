use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SimulatedRobotWithView {
    pub identifier: Option<String>,
    pub initial_pose: Option<Vec<f64>>,
    #[serde(alias = "type")]
    pub type_name: Option<String>,
}
