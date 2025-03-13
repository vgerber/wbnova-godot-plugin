use crate::objects::geometry::Geometry;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Compound {
    pub child_geometries: Vec<Geometry>,
}
