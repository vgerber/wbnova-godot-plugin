use reqwest;

pub enum DeleteStoredCollisionSceneResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteStoredCollisionScenePathParameters {
    pub scene: String,
    pub cell: String,
}

pub async fn delete_stored_collision_scene(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteStoredCollisionScenePathParameters,
) -> Result<DeleteStoredCollisionSceneResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
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
        _ => Ok(DeleteStoredCollisionSceneResponseType::UndefinedResponse(
            response,
        )),
    }
}
