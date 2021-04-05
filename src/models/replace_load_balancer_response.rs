/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ReplaceLoadBalancerResponse : Response to PUT https://api.hetzner.cloud/v1/load_balancers/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplaceLoadBalancerResponse {
    #[serde(rename = "load_balancer")]
    pub load_balancer: crate::models::LoadBalancer,
}

impl ReplaceLoadBalancerResponse {
    /// Response to PUT https://api.hetzner.cloud/v1/load_balancers/{id}
    pub fn new(load_balancer: crate::models::LoadBalancer) -> ReplaceLoadBalancerResponse {
        ReplaceLoadBalancerResponse {
            load_balancer,
        }
    }
}


