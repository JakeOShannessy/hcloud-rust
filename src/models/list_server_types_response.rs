/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListServerTypesResponse {
    #[serde(rename = "server_types")]
    pub server_types: Vec<crate::models::ServerType>,
}

impl ListServerTypesResponse {
    pub fn new(server_types: Vec<crate::models::ServerType>) -> ListServerTypesResponse {
        ListServerTypesResponse {
            server_types,
        }
    }
}


