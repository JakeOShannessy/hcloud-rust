/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AttachIsoToServerResponse : Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_iso



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachIsoToServerResponse {
    #[serde(rename = "action")]
    pub action: Box<crate::models::Action>,
}

impl AttachIsoToServerResponse {
    /// Response to POST https://api.hetzner.cloud/v1/servers/{id}/actions/attach_iso
    pub fn new(action: crate::models::Action) -> AttachIsoToServerResponse {
        AttachIsoToServerResponse {
            action: Box::new(action),
        }
    }
}


