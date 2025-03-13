use reqwest;

pub enum SetDefaultModeResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct SetDefaultModePathParameters {
    pub controller: String,
    pub cell: String,
}
pub struct SetDefaultModeQueryParameters {
    pub mode: String,
}

pub async fn set_default_mode(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &SetDefaultModePathParameters,
    query_parameters: &SetDefaultModeQueryParameters,
) -> Result<SetDefaultModeResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> =
        vec![("mode", query_parameters.mode.to_string())];
    let response = match client
        .put(format!(
            "{server}/cells/{}/controllers/{}/mode",
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
        _ => Ok(SetDefaultModeResponseType::UndefinedResponse(response)),
    }
}
