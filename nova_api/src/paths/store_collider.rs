use crate::objects::collider::Collider;
use reqwest;

pub enum StoreColliderResponseType {
    Ok(Collider),
    UndefinedResponse(reqwest::Response),
}

pub struct StoreColliderPathParameters {
    pub collider: String,
    pub cell: String,
}

pub async fn store_collider(
    client: &reqwest::Client,
    server: &str,
    content: Collider,
    path_parameters: &StoreColliderPathParameters,
) -> Result<StoreColliderResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/store/collision/colliders/{}",
            path_parameters.collider, path_parameters.cell
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
        200 => match response.json::<Collider>().await {
            Ok(collider) => Ok(StoreColliderResponseType::Ok(collider)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(StoreColliderResponseType::UndefinedResponse(response)),
    }
}
