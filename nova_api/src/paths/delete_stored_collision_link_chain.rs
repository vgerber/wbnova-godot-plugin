use reqwest;

pub enum DeleteStoredCollisionLinkChainResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteStoredCollisionLinkChainPathParameters {
    pub cell: String,
    pub link_chain: String,
}

pub async fn delete_stored_collision_link_chain(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteStoredCollisionLinkChainPathParameters,
) -> Result<DeleteStoredCollisionLinkChainResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/store/collision/link-chains/{}",
            path_parameters.cell, path_parameters.link_chain
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(DeleteStoredCollisionLinkChainResponseType::UndefinedResponse(response)),
    }
}
