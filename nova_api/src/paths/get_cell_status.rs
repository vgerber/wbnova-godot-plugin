use crate::objects::error::Error;
use crate::objects::service_status_list::ServiceStatusList;
use reqwest;

pub enum GetCellStatusResponseType {
    Ok(ServiceStatusList),
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct GetCellStatusPathParameters {
    pub cell: String,
}

pub async fn get_cell_status(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetCellStatusPathParameters,
) -> Result<GetCellStatusResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/cells/{}/status", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<ServiceStatusList>().await {
            Ok(service_status_list) => Ok(GetCellStatusResponseType::Ok(service_status_list)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetCellStatusResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetCellStatusResponseType::UndefinedResponse(response)),
    }
}
