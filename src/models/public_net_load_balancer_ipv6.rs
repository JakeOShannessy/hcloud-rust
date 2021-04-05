/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PublicNetLoadBalancerIpv6 : IP address (v6)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicNetLoadBalancerIpv6 {
    /// IP address (v6) of this Load Balancer
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

impl PublicNetLoadBalancerIpv6 {
    /// IP address (v6)
    pub fn new() -> PublicNetLoadBalancerIpv6 {
        PublicNetLoadBalancerIpv6 {
            ip: None,
        }
    }
}


