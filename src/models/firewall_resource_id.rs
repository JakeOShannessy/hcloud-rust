/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// FirewallResourceId : Resource a Firewall should be applied to.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FirewallResourceId {
    /// Type of resource referenced
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<crate::models::FirewallResourceIdServer>,
}

impl FirewallResourceId {
    /// Resource a Firewall should be applied to.
    pub fn new(_type: Type) -> FirewallResourceId {
        FirewallResourceId {
            _type,
            server: None,
        }
    }
}

/// Type of resource referenced
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "server")]
    Server,
}

