/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Protection : Protection configuration for the Server



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Protection {
    /// If true, prevents the Server from being deleted
    #[serde(rename = "delete")]
    pub delete: bool,
    /// If true, prevents the Server from being rebuilt
    #[serde(rename = "rebuild")]
    pub rebuild: bool,
}

impl Protection {
    /// Protection configuration for the Server
    pub fn new(delete: bool, rebuild: bool) -> Protection {
        Protection {
            delete,
            rebuild,
        }
    }
}


