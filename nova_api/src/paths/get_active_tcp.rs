use crate::objects::robot_tcp::RobotTcp;
use reqwest;

pub enum GetActiveTcpResponseType {
    Ok(RobotTcp),
    UndefinedResponse(reqwest::Response),
}

pub struct GetActiveTcpPathParameters {
    pub motion_group: String,
    pub cell: String,
}
pub struct GetActiveTcpQueryParameters {
    pub rotation_type: Option<String>,
}

pub async fn get_active_tcp(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetActiveTcpPathParameters,
    query_parameters: &GetActiveTcpQueryParameters,
) -> Result<GetActiveTcpResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.rotation_type {
        request_query_parameters.push(("rotation_type", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/motion-groups/{}/tcps/current",
            path_parameters.motion_group, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<RobotTcp>().await {
            Ok(robot_tcp) => Ok(GetActiveTcpResponseType::Ok(robot_tcp)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetActiveTcpResponseType::UndefinedResponse(response)),
    }
}
