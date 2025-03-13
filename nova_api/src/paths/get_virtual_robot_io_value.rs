use crate::objects::io::Io;
use reqwest;

pub enum GetVirtualRobotIoValueResponseType {
    Ok(Io),
    UndefinedResponse(reqwest::Response),
}

pub struct GetVirtualRobotIoValuePathParameters {
    pub cell: String,
    pub controller: String,
    pub io: String,
}

pub async fn get_virtual_robot_io_value(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetVirtualRobotIoValuePathParameters,
) -> Result<GetVirtualRobotIoValueResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/ios/{}",
            path_parameters.cell, path_parameters.controller, path_parameters.io
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<Io>().await {
            Ok(io) => Ok(GetVirtualRobotIoValueResponseType::Ok(io)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetVirtualRobotIoValueResponseType::UndefinedResponse(
            response,
        )),
    }
}
