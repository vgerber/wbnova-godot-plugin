use reqwest;

pub enum DeleteStoredColliderResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteStoredColliderPathParameters {
    pub cell: String,
    pub collider: String,
}

pub async fn delete_stored_collider(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteStoredColliderPathParameters,
) -> Result<DeleteStoredColliderResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
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
        _ => Ok(DeleteStoredColliderResponseType::UndefinedResponse(
            response,
        )),
    }
}
