use crate::objects::http_exception_response::HttpExceptionResponse;
use crate::objects::http_validation_error::HttpValidationError;
use crate::objects::recipe_metadata::RecipeMetadata;
use reqwest;

pub enum UploadRecipeMetadataImageResponseType {
    Ok(RecipeMetadata),
    NotFound(HttpExceptionResponse),
    UnprocessableEntity(HttpValidationError),
    UndefinedResponse(reqwest::Response),
}

pub struct UploadRecipeMetadataImagePathParameters {
    pub cell: String,
    pub recipe: String,
}

pub async fn upload_recipe_metadata_image(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &UploadRecipeMetadataImagePathParameters,
) -> Result<UploadRecipeMetadataImageResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .post(format!(
            "{server}/cells/{}/store/recipes/{}/metadata/image",
            path_parameters.cell, path_parameters.recipe
        ))
        .query(&request_query_parameters)
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match response.status().as_u16() {
        200 => match response.json::<RecipeMetadata>().await {
            Ok(recipe_metadata) => Ok(UploadRecipeMetadataImageResponseType::Ok(recipe_metadata)),
            Err(parsing_error) => Err(parsing_error),
        },
        404 => match response.json::<HttpExceptionResponse>().await {
            Ok(http_exception_response) => Ok(UploadRecipeMetadataImageResponseType::NotFound(
                http_exception_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        422 => match response.json::<HttpValidationError>().await {
            Ok(http_validation_error) => Ok(
                UploadRecipeMetadataImageResponseType::UnprocessableEntity(http_validation_error),
            ),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(UploadRecipeMetadataImageResponseType::UndefinedResponse(
            response,
        )),
    }
}
