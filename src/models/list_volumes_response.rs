/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListVolumesResponse : Response to GET https://api.hetzner.cloud/v1/volumes



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListVolumesResponse {
    #[serde(rename = "volumes")]
    pub volumes: Vec<crate::models::Volume>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::Meta>>,
}

impl ListVolumesResponse {
    /// Response to GET https://api.hetzner.cloud/v1/volumes
    pub fn new(volumes: Vec<crate::models::Volume>) -> ListVolumesResponse {
        ListVolumesResponse {
            volumes,
            meta: None,
        }
    }
}


