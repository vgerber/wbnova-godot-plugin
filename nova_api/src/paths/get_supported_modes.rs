use crate::objects::controller_capabilities::ControllerCapabilities;
use reqwest;

pub enum GetSupportedModesResponseType {
    Ok(ControllerCapabilities),
    UndefinedResponse(reqwest::Response),
}

pub struct GetSupportedModesPathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn get_supported_modes(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetSupportedModesPathParameters,
) -> Result<GetSupportedModesResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/controller-capabilities",
            path_parameters.controller, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<ControllerCapabilities>().await {
            Ok(controller_capabilities) => {
                Ok(GetSupportedModesResponseType::Ok(controller_capabilities))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetSupportedModesResponseType::UndefinedResponse(response)),
    }
}
