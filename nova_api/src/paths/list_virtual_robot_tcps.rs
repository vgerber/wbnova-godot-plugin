use crate::objects::robot_tcps::RobotTcps;
use reqwest;

pub enum ListVirtualRobotTcpsResponseType {
    Ok(RobotTcps),
    UndefinedResponse(reqwest::Response),
}

pub struct ListVirtualRobotTcpsPathParameters {
    pub controller: String,
    pub id: String,
    pub cell: String,
}

pub async fn list_virtual_robot_tcps(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListVirtualRobotTcpsPathParameters,
) -> Result<ListVirtualRobotTcpsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/{}/tcps",
            path_parameters.controller, path_parameters.id, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<RobotTcps>().await {
            Ok(robot_tcps) => Ok(ListVirtualRobotTcpsResponseType::Ok(robot_tcps)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListVirtualRobotTcpsResponseType::UndefinedResponse(
            response,
        )),
    }
}
