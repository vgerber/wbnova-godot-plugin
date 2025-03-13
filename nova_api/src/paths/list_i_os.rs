use crate::objects::i_os::IOs;
use reqwest;

pub enum ListIOsResponseType {
    Ok(IOs),
    UndefinedResponse(reqwest::Response),
}

pub struct ListIOsPathParameters {
    pub cell: String,
    pub controller: String,
}

pub async fn list_i_os(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListIOsPathParameters,
) -> Result<ListIOsResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/teach-pendant/ios",
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
        200 => match response.json::<IOs>().await {
            Ok(i_os) => Ok(ListIOsResponseType::Ok(i_os)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListIOsResponseType::UndefinedResponse(response)),
    }
}
