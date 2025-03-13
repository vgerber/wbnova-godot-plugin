use crate::objects::io_value::IoValue;
use reqwest;

pub enum SetOutputValuesResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct SetOutputValuesPathParameters {
    pub cell: String,
    pub controller: String,
}

pub async fn set_output_values(
    client: &reqwest::Client,
    server: &str,
    content: Vec<IoValue>,
    path_parameters: &SetOutputValuesPathParameters,
) -> Result<SetOutputValuesResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/ios/values",
            path_parameters.cell, path_parameters.controller
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(SetOutputValuesResponseType::UndefinedResponse(response)),
    }
}
