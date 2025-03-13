use crate::objects::collider::Collider;
use reqwest;

pub enum GetStoredColliderResponseType {
    Ok(Collider),
    UndefinedResponse(reqwest::Response),
}

pub struct GetStoredColliderPathParameters {
    pub cell: String,
    pub collider: String,
}

pub async fn get_stored_collider(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetStoredColliderPathParameters,
) -> Result<GetStoredColliderResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/collision/colliders/{}",
            path_parameters.cell, path_parameters.collider
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Collider>().await {
            Ok(collider) => Ok(GetStoredColliderResponseType::Ok(collider)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetStoredColliderResponseType::UndefinedResponse(response)),
    }
}
