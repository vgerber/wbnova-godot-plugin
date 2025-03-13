use crate::objects::coordinate_system::CoordinateSystem;
use reqwest;

pub enum SetVirtualRobotMountingResponseType {
    Ok(CoordinateSystem),
    UndefinedResponse(reqwest::Response),
}

pub struct SetVirtualRobotMountingPathParameters {
    pub cell: String,
    pub controller: String,
    pub id: String,
}

pub async fn set_virtual_robot_mounting(
    client: &reqwest::Client,
    server: &str,
    content: CoordinateSystem,
    path_parameters: &SetVirtualRobotMountingPathParameters,
) -> Result<SetVirtualRobotMountingResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/{}/mounting",
            path_parameters.cell, path_parameters.controller, path_parameters.id
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<CoordinateSystem>().await {
            Ok(coordinate_system) => Ok(SetVirtualRobotMountingResponseType::Ok(coordinate_system)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(SetVirtualRobotMountingResponseType::UndefinedResponse(
            response,
        )),
    }
}
