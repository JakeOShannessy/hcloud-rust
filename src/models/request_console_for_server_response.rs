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
pub struct RequestConsoleForServerResponse {
    /// URL of websocket proxy to use; this includes a token which is valid for a limited time only
    #[serde(rename = "wss_url")]
    pub wss_url: String,
    /// VNC password to use for this connection (this password only works in combination with a wss_url with valid token)
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "action")]
    pub action: crate::models::Action,
}

impl RequestConsoleForServerResponse {
    pub fn new(wss_url: String, password: String, action: crate::models::Action) -> RequestConsoleForServerResponse {
        RequestConsoleForServerResponse {
            wss_url,
            password,
            action,
        }
    }
}


