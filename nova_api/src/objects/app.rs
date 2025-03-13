use crate::objects::container_image::ContainerImage;
use crate::objects::container_resources::ContainerResources;
use crate::objects::container_storage::ContainerStorage;
use crate::objects::object::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct App {
    #[serde(alias = "containerImage")]
    pub container_image: ContainerImage,
    #[serde(alias = "appIcon")]
    pub app_icon: String,
    pub environment: Option<Vec<Object>>,
    pub port: Option<i32>,
    pub name: String,
    pub resources: Option<ContainerResources>,
    pub storage: Option<ContainerStorage>,
}
