use crate::objects::link_chain::LinkChain;
use reqwest;

pub enum GetStoredCollisionLinkChainResponseType {
    Ok(LinkChain),
    UndefinedResponse(reqwest::Response),
}

pub struct GetStoredCollisionLinkChainPathParameters {
    pub link_chain: String,
    pub cell: String,
}

pub async fn get_stored_collision_link_chain(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetStoredCollisionLinkChainPathParameters,
) -> Result<GetStoredCollisionLinkChainResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/collision/link-chains/{}",
            path_parameters.link_chain, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<LinkChain>().await {
            Ok(link_chain) => Ok(GetStoredCollisionLinkChainResponseType::Ok(link_chain)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetStoredCollisionLinkChainResponseType::UndefinedResponse(
            response,
        )),
    }
}
