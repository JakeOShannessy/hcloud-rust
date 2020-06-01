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
pub struct AssignFloatingIpToServerRequest {
    /// ID of the Server the Floating IP shall be assigned to
    #[serde(rename = "server")]
    pub server: i32,
}

impl AssignFloatingIpToServerRequest {
    pub fn new(server: i32) -> AssignFloatingIpToServerRequest {
        AssignFloatingIpToServerRequest {
            server,
        }
    }
}


