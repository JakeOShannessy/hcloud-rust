/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ChangeNetworkProtectionResponse : Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_protection



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeNetworkProtectionResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl ChangeNetworkProtectionResponse {
    /// Response to POST https://api.hetzner.cloud/v1/networks/{id}/actions/change_protection
    pub fn new(action: crate::models::Action) -> ChangeNetworkProtectionResponse {
        ChangeNetworkProtectionResponse {
            action: Box::new(action),
        }
    }
}


