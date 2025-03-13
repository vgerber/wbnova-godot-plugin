use reqwest;

pub enum GetSystemVersionResponseType {
    Ok(String),
    UndefinedResponse(reqwest::Response),
}

pub async fn get_system_version(
    client: &reqwest::Client,
    server: &str,
) -> Result<GetSystemVersionResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/system/version",))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.text().await {
            Ok(response_text) => Ok(GetSystemVersionResponseType::Ok(response_text)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetSystemVersionResponseType::UndefinedResponse(response)),
    }
}
