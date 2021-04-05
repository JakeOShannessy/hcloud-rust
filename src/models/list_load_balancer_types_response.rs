/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListLoadBalancerTypesResponse : Response to GET https://api.hetzner.cloud/v1/load_balancer_types



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListLoadBalancerTypesResponse {
    #[serde(rename = "load_balancer_types")]
    pub load_balancer_types: Vec<crate::models::LoadBalancerType>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListLoadBalancerTypesResponse {
    /// Response to GET https://api.hetzner.cloud/v1/load_balancer_types
    pub fn new(load_balancer_types: Vec<crate::models::LoadBalancerType>) -> ListLoadBalancerTypesResponse {
        ListLoadBalancerTypesResponse {
            load_balancer_types,
            meta: None,
        }
    }
}


