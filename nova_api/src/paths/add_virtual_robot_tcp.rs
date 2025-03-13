use crate::objects::empty::Empty;
use crate::objects::robot_tcp::RobotTcp;
use reqwest;

pub enum AddVirtualRobotTcpResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct AddVirtualRobotTcpPathParameters {
    pub controller: String,
    pub cell: String,
    pub id: String,
}

pub async fn add_virtual_robot_tcp(
    client: &reqwest::Client,
    server: &str,
    content: RobotTcp,
    path_parameters: &AddVirtualRobotTcpPathParameters,
) -> Result<AddVirtualRobotTcpResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/{}/tcps",
            path_parameters.controller, path_parameters.cell, path_parameters.id
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
            Ok(empty) => Ok(AddVirtualRobotTcpResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(AddVirtualRobotTcpResponseType::UndefinedResponse(response)),
    }
}
