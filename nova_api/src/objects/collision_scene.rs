use crate::objects::collider_dictionary::ColliderDictionary;
use crate::objects::collision_motion_group_dictionary::CollisionMotionGroupDictionary;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CollisionScene {
    pub motion_groups: Option<CollisionMotionGroupDictionary>,
    pub colliders: Option<ColliderDictionary>,
}
