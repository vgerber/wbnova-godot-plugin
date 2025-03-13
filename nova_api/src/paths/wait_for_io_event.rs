use reqwest;

pub enum WaitForIoEventResponseType {
    Ok(bool),
    UndefinedResponse(reqwest::Response),
}

pub struct WaitForIoEventPathParameters {
    pub controller: String,
    pub cell: String,
}
pub struct WaitForIoEventQueryParameters {
    pub integer_value: Option<String>,
    pub boolean_value: Option<bool>,
    pub io: String,
    pub comparison_type: String,
    pub floating_value: Option<f64>,
}

pub async fn wait_for_io_event(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &WaitForIoEventPathParameters,
    query_parameters: &WaitForIoEventQueryParameters,
) -> Result<WaitForIoEventResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![
        ("io", query_parameters.io.to_string()),
        (
            "comparison_type",
            query_parameters.comparison_type.to_string(),
        ),
    ];
    if let Some(ref query_parameter) = query_parameters.integer_value {
        request_query_parameters.push(("integer_value", query_parameter.to_string()));
    }
    if let Some(ref query_parameter) = query_parameters.boolean_value {
        request_query_parameters.push(("boolean_value", query_parameter.to_string()));
    }
    if let Some(ref query_parameter) = query_parameters.floating_value {
        request_query_parameters.push(("floating_value", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers/{}/ios/wait-for",
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
        200 => match response.json::<bool>().await {
            Ok(bool) => Ok(WaitForIoEventResponseType::Ok(bool)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(WaitForIoEventResponseType::UndefinedResponse(response)),
    }
}
