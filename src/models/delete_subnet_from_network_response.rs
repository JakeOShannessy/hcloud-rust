/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DeleteSubnetFromNetworkResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_subnet



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteSubnetFromNetworkResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl DeleteSubnetFromNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_subnet
    pub fn new(action: crate::models::Action) -> DeleteSubnetFromNetworkResponse {
        DeleteSubnetFromNetworkResponse {
            action,
        }
    }
}


