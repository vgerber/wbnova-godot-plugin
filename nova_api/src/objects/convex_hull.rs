use crate::objects::vector_3_d::Vector3D;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConvexHull {
    pub vertices: Vec<Vector3D>,
}
