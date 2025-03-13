use crate::objects::cell::Cell;
use crate::objects::error::Error;
use reqwest;

pub enum DeployCellResponseType {
    Forbidden(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct DeployCellQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn deploy_cell(
    client: &reqwest::Client,
    server: &str,
    content: Cell,
    query_parameters: &DeployCellQueryParameters,
) -> Result<DeployCellResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        request_query_parameters.push(("completionTimeout", query_parameter.to_string()));
    }
    let response = match client
        .post(format!("{server}/cells",))
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
            Ok(error) => Ok(DeployCellResponseType::Forbidden(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(DeployCellResponseType::UndefinedResponse(response)),
    }
}
