use crate::objects::coordinate_system::CoordinateSystem;
use crate::objects::empty::Empty;
use reqwest;

pub enum AddVirtualRobotCoordinateSystemResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct AddVirtualRobotCoordinateSystemPathParameters {
    pub controller: String,
    pub cell: String,
}

pub async fn add_virtual_robot_coordinate_system(
    client: &reqwest::Client,
    server: &str,
    content: CoordinateSystem,
    path_parameters: &AddVirtualRobotCoordinateSystemPathParameters,
) -> Result<AddVirtualRobotCoordinateSystemResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/coordinate-systems",
            path_parameters.controller, path_parameters.cell
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
        200 => match response.json::<Empty>().await {
            Ok(empty) => Ok(AddVirtualRobotCoordinateSystemResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(AddVirtualRobotCoordinateSystemResponseType::UndefinedResponse(response)),
    }
}
