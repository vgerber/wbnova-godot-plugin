use crate::objects::controller_instance_list::ControllerInstanceList;
use reqwest;

pub enum ListControllersResponseType {
    Ok(ControllerInstanceList),
    UndefinedResponse(reqwest::Response),
}

pub struct ListControllersPathParameters {
    pub cell: String,
}

pub async fn list_controllers(
    client: &reqwest::Client,
    server: &str,
    path_parameters: &ListControllersPathParameters,
) -> Result<ListControllersResponseType, reqwest::Error> {
    let request_query_parameters: Vec<(&str, String)> = vec![];
    let response = match client
        .get(format!(
            "{server}/cells/{}/controllers",
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
        200 => match response.json::<ControllerInstanceList>().await {
            Ok(controller_instance_list) => {
                Ok(ListControllersResponseType::Ok(controller_instance_list))
            }
            Err(parsing_error) => Err(parsing_error),
        },
        _ => Ok(ListControllersResponseType::UndefinedResponse(response)),
    }
}
