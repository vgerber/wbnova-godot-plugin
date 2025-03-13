use crate::objects::virtual_robot_configuration::VirtualRobotConfiguration;
use reqwest;

pub enum GetVirtualRobotConfigurationResponseType {
    Ok(VirtualRobotConfiguration),
    UndefinedResponse(reqwest::Response),
}

pub struct GetVirtualRobotConfigurationPathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn get_virtual_robot_configuration(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetVirtualRobotConfigurationPathParameters,
) -> Result<GetVirtualRobotConfigurationResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/virtual-robot-configuration",
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
        200 => match response.json::<VirtualRobotConfiguration>().await {
            Ok(virtual_robot_configuration) => Ok(GetVirtualRobotConfigurationResponseType::Ok(
                virtual_robot_configuration,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetVirtualRobotConfigurationResponseType::UndefinedResponse(
            response,
        )),
    }
}
