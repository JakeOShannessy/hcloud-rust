/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetServerResponse : Response to GET https://api.hetzner.cloud/v1/servers/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetServerResponse {
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<crate::models::Server>,
}

impl GetServerResponse {
    /// Response to GET https://api.hetzner.cloud/v1/servers/{id}
    pub fn new() -> GetServerResponse {
        GetServerResponse {
            server: None,
        }
    }
}


