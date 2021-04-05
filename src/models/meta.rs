/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Meta : Metadata contained in the response



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::Pagination>>,
}

impl Meta {
    /// Metadata contained in the response
    pub fn new() -> Meta {
        Meta {
            pagination: None,
        }
    }
}


