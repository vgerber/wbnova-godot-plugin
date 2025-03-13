use crate::objects::app::App;
use reqwest;

pub enum AddAppResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct AddAppPathParameters {
    pub cell: String,
}
pub struct AddAppQueryParameters {
    pub completion_timeout: Option<i32>,
}

pub async fn add_app(
    client: &reqwest::Client,
    server: &str,
    content: App,
    path_parameters: &AddAppPathParameters,
    query_parameters: &AddAppQueryParameters,
) -> Result<AddAppResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.completion_timeout {
        request_query_parameters.push(("completionTimeout", query_parameter.to_string()));
    }
    let response = match client
        .post(format!("{server}/cells/{}/apps", path_parameters.cell))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(AddAppResponseType::UndefinedResponse(response)),
    }
}
