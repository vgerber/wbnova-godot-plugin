use crate::objects::license_status::LicenseStatus;
use reqwest;

pub enum GetLicenseStatusResponseType {
    Ok(LicenseStatus),
    Forbidden(LicenseStatus),
    UndefinedResponse(reqwest::Response),
}

pub async fn get_license_status(
    client: &reqwest::Client,
    server: &str,
) -> Result<GetLicenseStatusResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/license/status",))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<LicenseStatus>().await {
            Ok(license_status) => Ok(GetLicenseStatusResponseType::Ok(license_status)),
            Err(parsing_error) => Err(parsing_error),
        },
        403 => match response.json::<LicenseStatus>().await {
            Ok(license_status) => Ok(GetLicenseStatusResponseType::Forbidden(license_status)),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetLicenseStatusResponseType::UndefinedResponse(response)),
    }
}
