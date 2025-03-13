use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::list_program_metadata_response::ListProgramMetadataResponse;
use reqwest;

pub enum ListProgramMetadataResponseType {
    Ok(ListProgramMetadataResponse),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct ListProgramMetadataPathParameters {
    pub cell: String,
}
pub struct ListProgramMetadataQueryParameters {
    pub show_hidden: Option<bool>,
}

pub async fn list_program_metadata(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListProgramMetadataPathParameters,
    query_parameters: &ListProgramMetadataQueryParameters,
) -> Result<ListProgramMetadataResponseType, reqwest::Error> {
    let mut request_query_parameters: Vec<(&str, String)> = vec![];
    if let Some(ref query_parameter) = query_parameters.show_hidden {
        request_query_parameters.push(("show_hidden", query_parameter.to_string()));
    }
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/programs",
            path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<ListProgramMetadataResponse>().await {
            Ok(list_program_metadata_response) => Ok(ListProgramMetadataResponseType::Ok(
                list_program_metadata_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(ListProgramMetadataResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListProgramMetadataResponseType::UndefinedResponse(response)),
    }
}
