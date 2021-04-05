/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetNetworkResponse : Response to GET https://api.hetzner.cloud/v1/networks/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkResponse {
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<crate::models::Network>,
}

impl GetNetworkResponse {
    /// Response to GET https://api.hetzner.cloud/v1/networks/{id}
    pub fn new() -> GetNetworkResponse {
        GetNetworkResponse {
            network: None,
        }
    }
}


