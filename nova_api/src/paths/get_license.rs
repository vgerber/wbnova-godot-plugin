use crate::objects::error::Error;
use crate::objects::license::License;
use reqwest;

pub enum GetLicenseResponseType {
    NotFound(Error),
    Ok(License),
    UndefinedResponse(reqwest::Response),
}

pub async fn get_license(
    client: &reqwest::Client,
    server: &str,
) -> Result<GetLicenseResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/license",))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        404 => match response.json::<Error>().await {
            Ok(error) => Ok(GetLicenseResponseType::NotFound(error)),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<License>().await {
            Ok(license) => Ok(GetLicenseResponseType::Ok(license)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetLicenseResponseType::UndefinedResponse(response)),
    }
}
