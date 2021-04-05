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
pub struct HealthStatus {
    #[serde(rename = "listen_port", skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl HealthStatus {
    pub fn new() -> HealthStatus {
        HealthStatus {
            listen_port: None,
            status: None,
        }
    }
}


