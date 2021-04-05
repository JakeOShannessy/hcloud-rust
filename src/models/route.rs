/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Route : Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/add_route | Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_route



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Route {
    /// Destination network or host of this route. Must not overlap with an existing ip_range in any subnets or with any destinations in other routes or with the first IP of the networks ip_range or with 172.31.1.1. Must be one of the private IPv4 ranges of RFC1918.
    #[serde(rename = "destination")]
    pub destination: String,
    /// Gateway for the route. Cannot be the first IP of the networks ip_range and also cannot be 172.31.1.1 as this IP is being used as a gateway for the public network interface of Servers. | Gateway for the route. Cannot be the first IP of the networks ip_range, an IP behind a vSwitch or 172.31.1.1, as this IP is being used as a gateway for the public network interface of Servers.
    #[serde(rename = "gateway")]
    pub gateway: String,
}

impl Route {
    /// Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/add_route | Request for POST https://api.hetzner.cloud/v1/networks/{id}/actions/delete_route
    pub fn new(destination: String, gateway: String) -> Route {
        Route {
            destination,
            gateway,
        }
    }
}


