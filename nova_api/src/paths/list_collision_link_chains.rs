use crate::objects::list_collision_link_chains_ok_json::ListCollisionLinkChainsOkJson;
use reqwest;

pub enum ListCollisionLinkChainsResponseType {
    Ok(ListCollisionLinkChainsOkJson),
    UndefinedResponse(reqwest::Response),
}

pub struct ListCollisionLinkChainsPathParameters {
    pub cell: String,
}

pub async fn list_collision_link_chains(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListCollisionLinkChainsPathParameters,
) -> Result<ListCollisionLinkChainsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/collision/link-chains",
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
        200 => match response.json::<ListCollisionLinkChainsOkJson>().await {
            Ok(list_collision_link_chains_ok_json) => Ok(ListCollisionLinkChainsResponseType::Ok(
                list_collision_link_chains_ok_json,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListCollisionLinkChainsResponseType::UndefinedResponse(
            response,
        )),
    }
}
