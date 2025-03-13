use crate::objects::activate_license_request_body_json::ActivateLicenseRequestBodyJson;
use crate::objects::error::Error;
use crate::objects::license::License;
use reqwest;

pub enum ActivateLicenseResponseType {
    Ok(License),
    NotFound(Error),
    Forbidden(Error),
    UndefinedResponse(reqwest::Response),
}

pub async fn activate_license(
    client: &reqwest::Client,
    server: &str,
    content: ActivateLicenseRequestBodyJson,
) -> Result<ActivateLicenseResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!("{server}/license",))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<License>().await {
            Ok(license) => Ok(ActivateLicenseResponseType::Ok(license)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(ActivateLicenseResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        403 => match response.json::<Error>().await {
            Ok(error) => Ok(ActivateLicenseResponseType::Forbidden(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ActivateLicenseResponseType::UndefinedResponse(response)),
    }
}
