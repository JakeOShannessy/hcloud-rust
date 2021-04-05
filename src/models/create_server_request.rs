/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateServerRequest : Request for POST https://api.hetzner.cloud/v1/servers



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateServerRequest {
    /// Name of the Server to create (must be unique per Project and a valid hostname as per RFC 1123)
    #[serde(rename = "name")]
    pub name: String,
    /// ID or name of Location to create Server in (must not be used together with datacenter)
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// ID or name of Datacenter to create Server in (must not be used together with location)
    #[serde(rename = "datacenter", skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
    /// ID or name of the Server type this Server should be created with
    #[serde(rename = "server_type")]
    pub server_type: String,
    /// Start Server right after creation. Defaults to true.
    #[serde(rename = "start_after_create", skip_serializing_if = "Option::is_none")]
    pub start_after_create: Option<bool>,
    /// ID or name of the Image the Server is created from
    #[serde(rename = "image")]
    pub image: String,
    /// SSH key IDs or names which should be injected into the Server at creation time
    #[serde(rename = "ssh_keys", skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<Vec<String>>,
    /// Volume IDs which should be attached to the Server at the creation time. Volumes must be in the same Location.
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<serde_json::Value>>,
    /// Network IDs which should be attached to the Server private network interface at the creation time
    #[serde(rename = "networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<serde_json::Value>>,
    /// Firewalls which should be applied on the Server's public network interface at creation time
    #[serde(rename = "firewalls", skip_serializing_if = "Option::is_none")]
    pub firewalls: Option<Vec<crate::models::CreateServerRequestFirewalls>>,
    /// Cloud-Init user data to use during Server creation. This field is limited to 32KiB.
    #[serde(rename = "user_data", skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// Auto-mount Volumes after attach
    #[serde(rename = "automount", skip_serializing_if = "Option::is_none")]
    pub automount: Option<bool>,
}

impl CreateServerRequest {
    /// Request for POST https://api.hetzner.cloud/v1/servers
    pub fn new(name: String, server_type: String, image: String) -> CreateServerRequest {
        CreateServerRequest {
            name,
            location: None,
            datacenter: None,
            server_type,
            start_after_create: None,
            image,
            ssh_keys: None,
            volumes: None,
            networks: None,
            firewalls: None,
            user_data: None,
            labels: None,
            automount: None,
        }
    }
}


