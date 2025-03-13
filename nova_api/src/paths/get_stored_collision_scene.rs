use crate::objects::collision_scene::CollisionScene;
use reqwest;

pub enum GetStoredCollisionSceneResponseType {
    Ok(CollisionScene),
    UndefinedResponse(reqwest::Response),
}

pub struct GetStoredCollisionScenePathParameters {
    pub scene: String,
    pub cell: String,
}

pub async fn get_stored_collision_scene(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetStoredCollisionScenePathParameters,
) -> Result<GetStoredCollisionSceneResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/collision/scenes/{}",
            path_parameters.scene, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<CollisionScene>().await {
            Ok(collision_scene) => Ok(GetStoredCollisionSceneResponseType::Ok(collision_scene)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetStoredCollisionSceneResponseType::UndefinedResponse(
            response,
        )),
    }
}
