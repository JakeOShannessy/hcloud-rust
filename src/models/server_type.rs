/*
 * Hetzner Cloud API
 *
 * Copied from the official API documentation for the Public Hetzner Cloud.
 *
 * The version of the OpenAPI document: 0.2.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ServerType : Type of Server - determines how much ram, disk and cpu a Server has



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerType {
    /// ID of the Server type
    #[serde(rename = "id")]
    pub id: i32,
    /// Unique identifier of the Server type
    #[serde(rename = "name")]
    pub name: String,
    /// Description of the Server type
    #[serde(rename = "description")]
    pub description: String,
    /// Number of cpu cores a Server of this type will have
    #[serde(rename = "cores")]
    pub cores: i32,
    /// Memory a Server of this type will have in GB
    #[serde(rename = "memory")]
    pub memory: f32,
    /// Disk size a Server of this type will have in GB
    #[serde(rename = "disk")]
    pub disk: f32,
    /// True if Server type is deprecated
    #[serde(rename = "deprecated")]
    pub deprecated: Option<bool>,
    /// Prices in different Locations
    #[serde(rename = "prices")]
    pub prices: Vec<crate::models::PricePerTime>,
    /// Type of Server boot drive. Local has higher speed. Network has better availability.
    #[serde(rename = "storage_type")]
    pub storage_type: StorageType,
    /// Type of cpu
    #[serde(rename = "cpu_type")]
    pub cpu_type: CpuType,
}

impl ServerType {
    /// Type of Server - determines how much ram, disk and cpu a Server has
    pub fn new(id: i32, name: String, description: String, cores: i32, memory: f32, disk: f32, deprecated: Option<bool>, prices: Vec<crate::models::PricePerTime>, storage_type: StorageType, cpu_type: CpuType) -> ServerType {
        ServerType {
            id,
            name,
            description,
            cores,
            memory,
            disk,
            deprecated,
            prices,
            storage_type,
            cpu_type,
        }
    }
}

/// Type of Server boot drive. Local has higher speed. Network has better availability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StorageType {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "network")]
    Network,
}
/// Type of cpu
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CpuType {
    #[serde(rename = "dedicated")]
    Dedicated,
    #[serde(rename = "shared")]
    Shared,
}

