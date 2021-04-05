/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DetachLoadBalancerFromNetworkResponse : Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/detach_from_network



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetachLoadBalancerFromNetworkResponse {
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl DetachLoadBalancerFromNetworkResponse {
    /// Response to POST https://api.hetzner.cloud/v1/load_balancers/{id}/actions/detach_from_network
    pub fn new(action: crate::models::Action) -> DetachLoadBalancerFromNetworkResponse {
        DetachLoadBalancerFromNetworkResponse {
            action,
        }
    }
}


