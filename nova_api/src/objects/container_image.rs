use crate::objects::image_credentials::ImageCredentials;
use crate::objects::object::Object;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ContainerImage {
    pub credentials: Option<ImageCredentials>,
    pub image: String,
    pub secrets: Option<Vec<Object>>,
}
