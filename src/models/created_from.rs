/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CreatedFrom : Information about the Server the Image was created from



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatedFrom {
    /// ID of the Server the Image was created from
    #[serde(rename = "id")]
    pub id: i32,
    /// Server name at the time the Image was created
    #[serde(rename = "name")]
    pub name: String,
}

impl CreatedFrom {
    /// Information about the Server the Image was created from
    pub fn new(id: i32, name: String) -> CreatedFrom {
        CreatedFrom {
            id,
            name,
        }
    }
}


