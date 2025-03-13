use crate::objects::collision_scene::CollisionScene;
use crate::objects::collision_scene_assembly::CollisionSceneAssembly;
use reqwest;

pub enum StoreCollisionSceneResponseType {
    Ok(CollisionScene),
    UndefinedResponse(reqwest::Response),
}

pub struct StoreCollisionScenePathParameters {
    pub cell: String,
    pub scene: String,
}

pub async fn store_collision_scene(
    client: &reqwest::Client,
    server: &str,
    content: CollisionSceneAssembly,
    path_parameters: &StoreCollisionScenePathParameters,
) -> Result<StoreCollisionSceneResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/store/collision/scenes/{}",
            path_parameters.cell, path_parameters.scene
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<CollisionScene>().await {
            Ok(collision_scene) => Ok(StoreCollisionSceneResponseType::Ok(collision_scene)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(StoreCollisionSceneResponseType::UndefinedResponse(response)),
    }
}
