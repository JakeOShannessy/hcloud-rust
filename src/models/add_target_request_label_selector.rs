/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AddTargetRequestLabelSelector : Configuration for label selector targets, required if type is `label_selector`



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddTargetRequestLabelSelector {
    /// Label selector
    #[serde(rename = "selector")]
    pub selector: String,
}

impl AddTargetRequestLabelSelector {
    /// Configuration for label selector targets, required if type is `label_selector`
    pub fn new(selector: String) -> AddTargetRequestLabelSelector {
        AddTargetRequestLabelSelector {
            selector,
        }
    }
}


