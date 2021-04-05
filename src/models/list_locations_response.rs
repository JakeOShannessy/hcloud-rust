/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListLocationsResponse : Response to GET https://api.hetzner.cloud/v1/locations



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListLocationsResponse {
    #[serde(rename = "locations")]
    pub locations: Vec<crate::models::Location>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::models::Meta>,
}

impl ListLocationsResponse {
    /// Response to GET https://api.hetzner.cloud/v1/locations
    pub fn new(locations: Vec<crate::models::Location>) -> ListLocationsResponse {
        ListLocationsResponse {
            locations,
            meta: None,
        }
    }
}


