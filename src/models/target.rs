/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Target : A target for a load balancer



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Target {
    /// Type of the resource
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<crate::models::TargetServer>,
    /// List of health statuses of the services on this target
    #[serde(rename = "health_status", skip_serializing_if = "Option::is_none")]
    pub health_status: Option<Vec<crate::models::HealthStatus>>,
    /// Use the private network IP instead of the public IP. Default value is false.
    #[serde(rename = "use_private_ip", skip_serializing_if = "Option::is_none")]
    pub use_private_ip: Option<bool>,
    #[serde(rename = "label_selector", skip_serializing_if = "Option::is_none")]
    pub label_selector: Option<crate::models::TargetLabelSelector>,
    #[serde(rename = "ip", skip_serializing_if = "Option::is_none")]
    pub ip: Option<crate::models::AddTargetRequestIp>,
    /// List of selected targets
    #[serde(rename = "targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<crate::models::SelectedTarget>>,
}

impl Target {
    /// A target for a load balancer
    pub fn new(_type: Type) -> Target {
        Target {
            _type,
            server: None,
            health_status: None,
            use_private_ip: None,
            label_selector: None,
            ip: None,
            targets: None,
        }
    }
}

/// Type of the resource
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "label_selector")]
    LabelSelector,
    #[serde(rename = "server")]
    Server,
}

