use crate::objects::collider_dictionary::ColliderDictionary;
use reqwest;

pub enum ListStoredCollidersResponseType {
    Ok(ColliderDictionary),
    UndefinedResponse(reqwest::Response),
}

pub struct ListStoredCollidersPathParameters {
    pub cell: String,
}

pub async fn list_stored_colliders(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListStoredCollidersPathParameters,
) -> Result<ListStoredCollidersResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/collision/colliders",
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
        200 => match response.json::<ColliderDictionary>().await {
            Ok(collider_dictionary) => Ok(ListStoredCollidersResponseType::Ok(collider_dictionary)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListStoredCollidersResponseType::UndefinedResponse(response)),
    }
}
