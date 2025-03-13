use crate::objects::collider_dictionary::ColliderDictionary;
use crate::objects::collision_scene::CollisionScene;
use crate::objects::map_motion_group_to_collision_motion_group_assembly::MapMotionGroupToCollisionMotionGroupAssembly;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CollisionSceneAssembly {
    pub motion_groups: Option<MapMotionGroupToCollisionMotionGroupAssembly>,
    pub stored_colliders: Option<Vec<String>>,
    pub stored_scenes: Option<Vec<String>>,
    pub colliders: Option<ColliderDictionary>,
    pub scene: Option<CollisionScene>,
}
