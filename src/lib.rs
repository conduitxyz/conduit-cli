pub mod api;
pub mod handler;
pub mod opts;
pub mod utils;

// Generate the protobuf types.
pub mod types {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));

    use serde::de::{self, Deserialize, Deserializer};
    pub fn deployment_type_from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        Ok(match s {
            "DEPLOYMENTTYPE_ANVIL" => DeploymentType::DeploymenttypeAnvil as i32,
            "DEPLOYMENTTYPE_ERIGON" => DeploymentType::DeploymenttypeErigon as i32,
            "DEPLOYMENTTYPE_GETH" => DeploymentType::DeploymenttypeGeth as i32,
            _ => return Err(de::Error::unknown_variant(s, &["anvil", "erigon", "geth"])),
        })
    }
}
