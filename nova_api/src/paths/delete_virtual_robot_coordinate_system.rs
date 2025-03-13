use crate::objects::empty::Empty;
use reqwest;

pub enum DeleteVirtualRobotCoordinateSystemResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteVirtualRobotCoordinateSystemPathParameters {
    pub controller: String,
    pub coordinate_system: String,
    pub cell: String,
}
pub struct DeleteVirtualRobotCoordinateSystemQueryParameters {
    pub delete_dependent: Option<bool>,
}

pub async fn delete_virtual_robot_coordinate_system(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteVirtualRobotCoordinateSystemPathParameters,
    query_parameters: &DeleteVirtualRobotCoordinateSystemQueryParameters,
) -> Result<DeleteVirtualRobotCoordinateSystemResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.delete_dependent {
        request_query_parameters.push(("delete_dependent", query_parameter.to_string()));
    }
    let response = match client
        .delete(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/coordinate-systems/{}",
            path_parameters.controller, path_parameters.coordinate_system, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Empty>().await {
            Ok(empty) => Ok(DeleteVirtualRobotCoordinateSystemResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DeleteVirtualRobotCoordinateSystemResponseType::UndefinedResponse(response)),
    }
}
