use reqwest;

pub enum GetDiagnosePackageResponseType {
    UndefinedResponse(reqwest::Response),
}

pub async fn get_diagnose_package(
    client: &reqwest::Client,
    server: &str,
) -> Result<GetDiagnosePackageResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!("{server}/system/diagnosis-package/zip",))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        _ => Ok(GetDiagnosePackageResponseType::UndefinedResponse(response)),
    }
}
