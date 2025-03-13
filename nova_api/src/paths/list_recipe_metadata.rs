use crate::objects::list_recipe_metadata_response::ListRecipeMetadataResponse;
use reqwest;

pub enum ListRecipeMetadataResponseType {
    Ok(ListRecipeMetadataResponse),
    UndefinedResponse(reqwest::Response),
}

pub struct ListRecipeMetadataPathParameters {
    pub cell: String,
}

pub async fn list_recipe_metadata(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListRecipeMetadataPathParameters,
) -> Result<ListRecipeMetadataResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/store/recipes",
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
        200 => match response.json::<ListRecipeMetadataResponse>().await {
            Ok(list_recipe_metadata_response) => Ok(ListRecipeMetadataResponseType::Ok(
                list_recipe_metadata_response,
            )),
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListRecipeMetadataResponseType::UndefinedResponse(response)),
    }
}
