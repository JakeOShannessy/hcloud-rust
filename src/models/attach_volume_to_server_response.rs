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
pub struct AttachVolumeToServerResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl AttachVolumeToServerResponse {
    pub fn new(action: crate::models::Action) -> AttachVolumeToServerResponse {
        AttachVolumeToServerResponse {
            action,
        }
    }
}


