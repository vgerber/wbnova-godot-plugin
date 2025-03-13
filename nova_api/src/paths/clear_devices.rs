use reqwest;

pub enum ClearDevicesResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct ClearDevicesPathParameters {
    pub cell: String,
}

pub async fn clear_devices(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ClearDevicesPathParameters,
) -> Result<ClearDevicesResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!("{server}/cells/{}/devices", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(ClearDevicesResponseType::UndefinedResponse(response)),
    }
}
