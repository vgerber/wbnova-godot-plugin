use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RobotControllerConfiguration {
    pub rae_port: Option<i32>,
    #[serde(alias = "type")]
    pub type_name: Option<String>,
    pub host: Option<String>,
    pub identifier: Option<String>,
    pub rae_host: Option<String>,
    pub controller_model_name: Option<String>,
}
