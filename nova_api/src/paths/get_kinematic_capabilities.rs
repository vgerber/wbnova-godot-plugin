use crate::objects::kinematic_service_capabilities::KinematicServiceCapabilities;
use reqwest;

pub enum GetKinematicCapabilitiesResponseType {
    Ok(KinematicServiceCapabilities),
    UndefinedResponse(reqwest::Response),
}

pub struct GetKinematicCapabilitiesPathParameters {
    pub cell: String,
    pub motion_group: String,
}

pub async fn get_kinematic_capabilities(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetKinematicCapabilitiesPathParameters,
) -> Result<GetKinematicCapabilitiesResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/kinematic-capabilities",
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
        200 => match response.json::<KinematicServiceCapabilities>().await {
            Ok(kinematic_service_capabilities) => Ok(GetKinematicCapabilitiesResponseType::Ok(
                kinematic_service_capabilities,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetKinematicCapabilitiesResponseType::UndefinedResponse(
            response,
        )),
    }
}
