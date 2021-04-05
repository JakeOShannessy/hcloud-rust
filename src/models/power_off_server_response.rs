/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PowerOffServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/poweroff



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PowerOffServerResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl PowerOffServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/poweroff
    pub fn new(action: crate::models::Action) -> PowerOffServerResponse {
        PowerOffServerResponse {
            action,
        }
    }
}


