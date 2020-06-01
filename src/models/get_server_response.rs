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
pub struct GetServerResponse {
    #[serde(rename = "server")]
    pub server: crate::models::Server,
}

impl GetServerResponse {
    pub fn new(server: crate::models::Server) -> GetServerResponse {
        GetServerResponse {
            server,
        }
    }
}


