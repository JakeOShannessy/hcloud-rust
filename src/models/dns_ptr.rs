/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DnsPtr {
    /// Single IPv4 or IPv6 address | Single IPv6 address of this Server for which the reverse DNS entry has been set up
    #[serde(rename = "ip")]
    pub ip: String,
    /// DNS pointer for the specific IP address
    #[serde(rename = "dns_ptr")]
    pub dns_ptr: String,
}

impl DnsPtr {
    pub fn new(ip: String, dns_ptr: String) -> DnsPtr {
        DnsPtr {
            ip,
            dns_ptr,
        }
    }
}


