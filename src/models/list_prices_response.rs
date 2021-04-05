/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ListPricesResponse : Response to GET https://api.hetzner.cloud/v1/pricing



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPricesResponse {
    #[serde(rename = "pricing")]
    pub pricing: Box<crate::models::ListPricesResponsePricing>,
}

impl ListPricesResponse {
    /// Response to GET https://api.hetzner.cloud/v1/pricing
    pub fn new(pricing: crate::models::ListPricesResponsePricing) -> ListPricesResponse {
        ListPricesResponse {
            pricing: Box::new(pricing),
        }
    }
}


