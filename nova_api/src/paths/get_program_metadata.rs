use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::program_metadata::ProgramMetadata;
use reqwest;

pub enum GetProgramMetadataResponseType {
    NotFound(HttpExceptionResponse),
    Ok(ProgramMetadata),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct GetProgramMetadataPathParameters {
    pub program: String,
    pub cell: String,
}

pub async fn get_program_metadata(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &GetProgramMetadataPathParameters,
) -> Result<GetProgramMetadataResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/programs/{}/metadata",
            path_parameters.program, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => Ok(GetProgramMetadataResponseType::NotFound(
                http_exception_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<ProgramMetadata>().await {
            Ok(program_metadata) => Ok(GetProgramMetadataResponseType::Ok(program_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(GetProgramMetadataResponseType::UnprocessableEntity(
                http_validation_error,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(GetProgramMetadataResponseType::UndefinedResponse(response)),
    }
}
