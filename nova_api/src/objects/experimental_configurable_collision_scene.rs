use crate::objects::object::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExperimentalConfigurableCollisionScene {
    pub static_colliders: Option<Object>,
    pub identifier: Option<String>,
    pub robot_configurations: Option<Object>,
    #[serde(alias = "type")]
    pub type_name: Option<String>,
}
