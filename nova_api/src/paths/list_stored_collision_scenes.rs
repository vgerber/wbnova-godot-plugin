use crate::objects::list_stored_collision_scenes_ok_json::ListStoredCollisionScenesOkJson;
use reqwest;

pub enum ListStoredCollisionScenesResponseType {
    Ok(ListStoredCollisionScenesOkJson),
    UndefinedResponse(reqwest::Response),
}

pub struct ListStoredCollisionScenesPathParameters {
    pub cell: String,
}

pub async fn list_stored_collision_scenes(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListStoredCollisionScenesPathParameters,
) -> Result<ListStoredCollisionScenesResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/collision/scenes",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<ListStoredCollisionScenesOkJson>().await {
            Ok(list_stored_collision_scenes_ok_json) => Ok(
                ListStoredCollisionScenesResponseType::Ok(list_stored_collision_scenes_ok_json),
            ),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListStoredCollisionScenesResponseType::UndefinedResponse(
            response,
        )),
    }
}
