/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateNetworkRequest : Request for POST https://api.hetzner.cloud/v1/networks



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNetworkRequest {
    /// Name of the network
    #[serde(rename = "name")]
    pub name: String,
    /// IP range of the whole network which must span all included subnets. Must be one of the private IPv4 ranges of RFC1918. Minimum network size is /24. We highly recommend that you pick a larger network with a /16 netmask.
    #[serde(rename = "ip_range")]
    pub ip_range: String,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Array of subnets allocated.
    #[serde(rename = "subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<crate::models::Subnet>>,
    /// Array of routes set in this network. The destination of the route must be one of the private IPv4 ranges of RFC1918. The gateway must be a subnet/IP of the ip_range of the network object. The destination must not overlap with an existing ip_range in any subnets or with any destinations in other routes or with the first IP of the networks ip_range or with 172.31.1.1. The gateway cannot be the first IP of the networks ip_range and also cannot be 172.31.1.1.
    #[serde(rename = "routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<crate::models::Route>>,
}

impl CreateNetworkRequest {
    /// Request for POST https://api.hetzner.cloud/v1/networks
    pub fn new(name: String, ip_range: String) -> CreateNetworkRequest {
        CreateNetworkRequest {
            name,
            ip_range,
            labels: None,
            subnets: None,
            routes: None,
        }
    }
}


