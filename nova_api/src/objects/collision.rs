use crate::objects::collision_contact::CollisionContact;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Collision {
    pub id_of_a: Option<String>,
    pub normal_world_on_b: Option<Vec<f64>>,
    pub position_on_a: Option<CollisionContact>,
    pub position_on_b: Option<CollisionContact>,
    pub id_of_world: Option<String>,
    pub id_of_b: Option<String>,
}
