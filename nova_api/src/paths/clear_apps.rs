use crate::objects::error::Error;
use reqwest;

pub enum ClearAppsResponseType {
    NotFound(Error),
    UndefinedResponse(reqwest::Response),
}

pub struct ClearAppsPathParameters {
    pub cell: String,
}
pub struct ClearAppsQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn clear_apps(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ClearAppsPathParameters,
    query_parameters: &ClearAppsQueryParameters,
) -> Result<ClearAppsResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        request_query_parameters.push(("completionTimeout", query_parameter.to_string()));
    }
    let response = match client
        .delete(format!("{server}/cells/{}/apps", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ClearAppsResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ClearAppsResponseType::UndefinedResponse(response)),
    }
}
