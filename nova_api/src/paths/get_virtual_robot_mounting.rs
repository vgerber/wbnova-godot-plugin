use crate::objects::coordinate_system::CoordinateSystem;
use reqwest;

pub enum GetVirtualRobotMountingResponseType {
    Ok(CoordinateSystem),
    UndefinedResponse(reqwest::Response),
}

pub struct GetVirtualRobotMountingPathParameters {
    pub cell: String,
    pub id: String,
    pub controller: String,
}

pub async fn get_virtual_robot_mounting(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetVirtualRobotMountingPathParameters,
) -> Result<GetVirtualRobotMountingResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/{}/mounting",
            path_parameters.cell, path_parameters.id, path_parameters.controller
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<CoordinateSystem>().await {
            Ok(coordinate_system) => Ok(GetVirtualRobotMountingResponseType::Ok(coordinate_system)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetVirtualRobotMountingResponseType::UndefinedResponse(
            response,
        )),
    }
}
