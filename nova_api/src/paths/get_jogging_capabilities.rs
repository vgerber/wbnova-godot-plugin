use crate::objects::jogging_service_capabilities::JoggingServiceCapabilities;
use reqwest;

pub enum GetJoggingCapabilitiesResponseType {
    Ok(JoggingServiceCapabilities),
    UndefinedResponse(reqwest::Response),
}

pub struct GetJoggingCapabilitiesPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn get_jogging_capabilities(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetJoggingCapabilitiesPathParameters,
) -> Result<GetJoggingCapabilitiesResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/jogging-capabilities",
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
        200 => match response.json::<JoggingServiceCapabilities>().await {
            Ok(jogging_service_capabilities) => Ok(GetJoggingCapabilitiesResponseType::Ok(
                jogging_service_capabilities,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetJoggingCapabilitiesResponseType::UndefinedResponse(
            response,
        )),
    }
}
