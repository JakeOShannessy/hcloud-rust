/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNetworkResponse {
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<crate::models::Network>,
}

impl CreateNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/networks
    pub fn new() -> CreateNetworkResponse {
        CreateNetworkResponse {
            network: None,
        }
    }
}


