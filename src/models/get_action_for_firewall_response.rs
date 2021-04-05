/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetActionForFirewallResponse : Response to GET https://api.hetzner.cloud/v1/firewalls/{id}/actions/{action_id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetActionForFirewallResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl GetActionForFirewallResponse {
    /// Response to GET https://api.hetzner.cloud/v1/firewalls/{id}/actions/{action_id}
    pub fn new(action: crate::models::Action) -> GetActionForFirewallResponse {
        GetActionForFirewallResponse {
            action,
        }
    }
}


