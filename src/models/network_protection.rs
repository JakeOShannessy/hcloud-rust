/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// NetworkProtection : Protection configuration for the Network



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProtection {
    /// If true, prevents the Network from being deleted
    #[serde(rename = "delete")]
    pub delete: bool,
}

impl NetworkProtection {
    /// Protection configuration for the Network
    pub fn new(delete: bool) -> NetworkProtection {
        NetworkProtection {
            delete,
        }
    }
}


