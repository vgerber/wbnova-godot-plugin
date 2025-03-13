use crate::objects::list_io_values_response::ListIoValuesResponse;
use reqwest;

pub enum ListIoValuesResponseType {
    Ok(ListIoValuesResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct ListIoValuesPathParameters {
    pub controller: String,
    pub cell: String,
}
pub struct ListIoValuesQueryParameters {
    pub ios: Option<Vec<String>>,
}

pub async fn list_io_values(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListIoValuesPathParameters,
    query_parameters: &ListIoValuesQueryParameters,
) -> Result<ListIoValuesResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.ios {
        query_parameter.iter().for_each(|query_parameter_item| {
            request_query_parameters.push(("ios", query_parameter_item.to_string()))
        });
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/ios/values",
            path_parameters.controller, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<ListIoValuesResponse>().await {
            Ok(list_io_values_response) => {
                Ok(ListIoValuesResponseType::Ok(list_io_values_response))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListIoValuesResponseType::UndefinedResponse(response)),
    }
}
