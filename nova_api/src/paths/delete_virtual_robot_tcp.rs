use crate::objects::empty::Empty;
use reqwest;

pub enum DeleteVirtualRobotTcpResponseType {
    Ok(Empty),
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteVirtualRobotTcpPathParameters {
    pub controller: String,
    pub id: String,
    pub cell: String,
    pub tcp: String,
}

pub async fn delete_virtual_robot_tcp(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteVirtualRobotTcpPathParameters,
) -> Result<DeleteVirtualRobotTcpResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/motion-groups/{}/tcps/{}",
            path_parameters.controller,
            path_parameters.id,
            path_parameters.cell,
            path_parameters.tcp
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
            Ok(empty) => Ok(DeleteVirtualRobotTcpResponseType::Ok(empty)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DeleteVirtualRobotTcpResponseType::UndefinedResponse(
            response,
        )),
    }
}
