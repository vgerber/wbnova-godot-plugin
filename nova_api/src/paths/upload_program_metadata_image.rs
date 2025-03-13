use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::program_metadata::ProgramMetadata;
use reqwest;

pub enum UploadProgramMetadataImageResponseType {
    NotFound(HttpExceptionResponse),
    UnprocessableEntity(HttpValidationError),
    Ok(ProgramMetadata),
    UndefinedResponse(reqwest::Response),
}

pub struct UploadProgramMetadataImagePathParameters {
    pub cell: String,
    pub program: String,
}

pub async fn upload_program_metadata_image(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &UploadProgramMetadataImagePathParameters,
) -> Result<UploadProgramMetadataImageResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/store/programs/{}/metadata/image",
            path_parameters.cell, path_parameters.program
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
            Ok(http_exception_response) => Ok(UploadProgramMetadataImageResponseType::NotFound(
                http_exception_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(
                UploadProgramMetadataImageResponseType::UnprocessableEntity(http_validation_error),
            ),
            Err(parsing_error) => Err(parsing_error),
        },
        200 => match response.json::<ProgramMetadata>().await {
            Ok(program_metadata) => {
                Ok(UploadProgramMetadataImageResponseType::Ok(program_metadata))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(UploadProgramMetadataImageResponseType::UndefinedResponse(
            response,
        )),
    }
}
