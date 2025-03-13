use crate::objects::error::Error;
use reqwest;

pub enum DeleteCellResponseType {
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct DeleteCellPathParameters {
    pub cell: String,
}
pub struct DeleteCellQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn delete_cell(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &DeleteCellPathParameters,
    query_parameters: &DeleteCellQueryParameters,
) -> Result<DeleteCellResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        request_query_parameters.push(("completionTimeout", query_parameter.to_string()));
    }
    let response = match client
        .delete(format!("{server}/cells/{}", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(DeleteCellResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DeleteCellResponseType::UndefinedResponse(response)),
    }
}
