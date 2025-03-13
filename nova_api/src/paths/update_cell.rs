use crate::objects::cell::Cell;
use crate::objects::error::Error;
use reqwest;

pub enum UpdateCellResponseType {
    Forbidden(Error),
    BadRequest(Error),
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct UpdateCellPathParameters {
    pub cell: String,
}
pub struct UpdateCellQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn update_cell(
    client: &reqwest::Client,
    server: &str,
    content: Cell,
    path_parameters: &UpdateCellPathParameters,
    query_parameters: &UpdateCellQueryParameters,
) -> Result<UpdateCellResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        request_query_parameters.push(("completionTimeout", query_parameter.to_string()));
    }
    let response = match client
        .put(format!("{server}/cells/{}", path_parameters.cell))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        403 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateCellResponseType::Forbidden(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        400 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateCellResponseType::BadRequest(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(UpdateCellResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(UpdateCellResponseType::UndefinedResponse(response)),
    }
}
