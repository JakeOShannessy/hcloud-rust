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
pub struct SubnetWithGateway {
    /// Type of Subnetwork
    #[serde(rename = "type")]
    pub _type: Type,
    /// Range to allocate IPs from. Must be a Subnet of the ip_range of the parent network object and must not overlap with any other subnets or with any destinations in routes. Minimum Network size is /30. We suggest that you pick a bigger Network with a /24 netmask.
    #[serde(rename = "ip_range", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    /// Name of Network zone. Currently eu-central is the only available zone.
    #[serde(rename = "network_zone")]
    pub network_zone: String,
    /// Gateway for Servers attached to this subnet. For subnets of type Server this is always the first IP of the network IP range.
    #[serde(rename = "gateway")]
    pub gateway: String,
}

impl SubnetWithGateway {
    pub fn new(_type: Type, network_zone: String, gateway: String) -> SubnetWithGateway {
        SubnetWithGateway {
            _type,
            ip_range: None,
            network_zone,
            gateway,
        }
    }
}

/// Type of Subnetwork
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "cloud")]
    Cloud,
    #[serde(rename = "server")]
    Server,
}

