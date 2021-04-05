/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreateSshKeyRequest : Request for POST https://api.hetzner.cloud/v1/ssh_keys



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSshKeyRequest {
    /// Name of the SSH key
    #[serde(rename = "name")]
    pub name: String,
    /// Public key
    #[serde(rename = "public_key")]
    pub public_key: String,
    /// User-defined labels (key-value pairs)
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
}

impl CreateSshKeyRequest {
    /// Request for POST https://api.hetzner.cloud/v1/ssh_keys
    pub fn new(name: String, public_key: String) -> CreateSshKeyRequest {
        CreateSshKeyRequest {
            name,
            public_key,
            labels: None,
        }
    }
}


