use crate::objects::coordinate_system::CoordinateSystem;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListResponse {
    pub coordinatesystems: Option<Vec<CoordinateSystem>>,
}
