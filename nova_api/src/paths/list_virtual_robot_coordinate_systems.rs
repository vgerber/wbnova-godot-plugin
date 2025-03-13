use crate::objects::coordinate_systems::CoordinateSystems;
use reqwest;

pub enum ListVirtualRobotCoordinateSystemsResponseType {
    Ok(CoordinateSystems),
    UndefinedResponse(reqwest::Response),
}

pub struct ListVirtualRobotCoordinateSystemsPathParameters {
    pub cell: String,
    pub controller: String,
}

pub async fn list_virtual_robot_coordinate_systems(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListVirtualRobotCoordinateSystemsPathParameters,
) -> Result<ListVirtualRobotCoordinateSystemsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/coordinate-systems",
            path_parameters.cell, path_parameters.controller
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<CoordinateSystems>().await {
            Ok(coordinate_systems) => Ok(ListVirtualRobotCoordinateSystemsResponseType::Ok(
                coordinate_systems,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListVirtualRobotCoordinateSystemsResponseType::UndefinedResponse(response)),
    }
}
