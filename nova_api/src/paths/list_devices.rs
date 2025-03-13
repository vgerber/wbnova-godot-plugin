use reqwest;

pub enum ListDevicesResponseType {
    UndefinedResponse(reqwest::Response),
}

pub struct ListDevicesPathParameters {
    pub cell: String,
}

pub async fn list_devices(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListDevicesPathParameters,
) -> Result<ListDevicesResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/cells/{}/devices", path_parameters.cell))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(ListDevicesResponseType::UndefinedResponse(response)),
    }
}
