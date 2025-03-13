use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::program_metadata::ProgramMetadata;
use crate::objects::update_program_metadata_request::UpdateProgramMetadataRequest;
use reqwest;

pub enum UpdateProgramMetadataResponseType {
    UnprocessableEntity(HttpValidationError),
    Ok(ProgramMetadata),
    NotFound(HttpExceptionResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct UpdateProgramMetadataPathParameters {
    pub program: String,
    pub cell: String,
}

pub async fn update_program_metadata(
    client: &reqwest::Client,
    server: &str,
    content: UpdateProgramMetadataRequest,
    path_parameters: &UpdateProgramMetadataPathParameters,
) -> Result<UpdateProgramMetadataResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .put(format!(
            "{server}/cells/{}/store/programs/{}/metadata",
            path_parameters.program, path_parameters.cell
        ))
        .query(&request_query_parameters)
        .json(&content)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(
                UpdateProgramMetadataResponseType::UnprocessableEntity(http_validation_error),
            ),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<ProgramMetadata>().await {
            Ok(program_metadata) => Ok(UpdateProgramMetadataResponseType::Ok(program_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => Ok(UpdateProgramMetadataResponseType::NotFound(
                http_exception_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(UpdateProgramMetadataResponseType::UndefinedResponse(
            response,
        )),
    }
}
