use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::link_chain::LinkChain;
use reqwest;

pub enum GetDefaultLinkChainResponseType {
    Ok(LinkChain),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct GetDefaultLinkChainPathParameters {
    pub motion_group_model: String,
    pub cell: String,
}

pub async fn get_default_link_chain(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetDefaultLinkChainPathParameters,
) -> Result<GetDefaultLinkChainResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/collision/default-link-chains/{}",
            path_parameters.motion_group_model, path_parameters.cell
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
            Ok(link_chain) => Ok(GetDefaultLinkChainResponseType::Ok(link_chain)),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(GetDefaultLinkChainResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetDefaultLinkChainResponseType::UndefinedResponse(response)),
    }
}
