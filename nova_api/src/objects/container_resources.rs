use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ContainerResources {
    #[serde(alias = "intel-gpu")]
    pub intel_gpu: Option<i32>,
}
