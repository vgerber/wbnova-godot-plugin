use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ContainerStorage {
    #[serde(alias = "mountPath")]
    pub mount_path: String,
    pub capacity: String,
}
