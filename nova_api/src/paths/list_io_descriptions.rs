use crate::objects::list_io_descriptions_response::ListIoDescriptionsResponse;
use reqwest;

pub enum ListIoDescriptionsResponseType {
    Ok(ListIoDescriptionsResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct ListIoDescriptionsPathParameters {
    pub cell: String,
    pub controller: String,
}
pub struct ListIoDescriptionsQueryParameters {
    pub ios: Option<Vec<String>>,
}

pub async fn list_io_descriptions(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListIoDescriptionsPathParameters,
    query_parameters: &ListIoDescriptionsQueryParameters,
) -> Result<ListIoDescriptionsResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.ios {
        query_parameter.iter().for_each(|query_parameter_item| {
            request_query_parameters.push(("ios", query_parameter_item.to_string()))
        });
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/ios/description",
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
        200 => match response.json::<ListIoDescriptionsResponse>().await {
            Ok(list_io_descriptions_response) => Ok(ListIoDescriptionsResponseType::Ok(
                list_io_descriptions_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListIoDescriptionsResponseType::UndefinedResponse(response)),
    }
}
