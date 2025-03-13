use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MotionGroupInfo {
    pub dof: i32,
    pub name: String,
    pub id: i32,
}
