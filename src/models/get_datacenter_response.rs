/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetDatacenterResponse : Response to GET https://api.hetzner.cloud/v1/datacenters/{id}



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetDatacenterResponse {
    #[serde(rename = "datacenter")]
    pub datacenter: crate::models::Datacenter,
}

impl GetDatacenterResponse {
    /// Response to GET https://api.hetzner.cloud/v1/datacenters/{id}
    pub fn new(datacenter: crate::models::Datacenter) -> GetDatacenterResponse {
        GetDatacenterResponse {
            datacenter,
        }
    }
}


