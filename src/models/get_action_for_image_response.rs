/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetActionForImageResponse : Response to GET https://api.hetzner.cloud/v1/images/{id}/actions/{action_id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActionForImageResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl GetActionForImageResponse {
    /// Response to GET https://api.hetzner.cloud/v1/images/{id}/actions/{action_id}
    pub fn new(action: crate::models::Action) -> GetActionForImageResponse {
        GetActionForImageResponse {
            action,
        }
    }
}


