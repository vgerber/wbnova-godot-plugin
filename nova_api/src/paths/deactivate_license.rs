use reqwest;

pub enum DeactivateLicenseResponseType {
    UndefinedResponse(reqwest::Response),
}

pub async fn deactivate_license(
    client: &reqwest::Client,
    server: &str,
) -> Result<DeactivateLicenseResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .delete(format!("{server}/license",))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(DeactivateLicenseResponseType::UndefinedResponse(response)),
    }
}
