use crate::objects::controller_instance::ControllerInstance;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ControllerInstanceList {
    pub instances: Vec<ControllerInstance>,
}
