use crate::objects::info_service_capabilities::InfoServiceCapabilities;
use reqwest;

pub enum GetInfoCapabilitiesResponseType {
    Ok(InfoServiceCapabilities),
    UndefinedResponse(reqwest::Response),
}

pub struct GetInfoCapabilitiesPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn get_info_capabilities(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetInfoCapabilitiesPathParameters,
) -> Result<GetInfoCapabilitiesResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/info-capabilities",
            path_parameters.cell, path_parameters.motion_group
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<InfoServiceCapabilities>().await {
            Ok(info_service_capabilities) => Ok(GetInfoCapabilitiesResponseType::Ok(
                info_service_capabilities,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetInfoCapabilitiesResponseType::UndefinedResponse(response)),
    }
}
