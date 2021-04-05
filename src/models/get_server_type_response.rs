/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetServerTypeResponse : Response to GET https://api.hetzner.cloud/v1/server_types/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetServerTypeResponse {
    #[serde(rename = "server_type")]
    pub server_type: Box<crate::models::ServerType>,
}

impl GetServerTypeResponse {
    /// Response to GET https://api.hetzner.cloud/v1/server_types/{id}
    pub fn new(server_type: crate::models::ServerType) -> GetServerTypeResponse {
        GetServerTypeResponse {
            server_type: Box::new(server_type),
        }
    }
}


