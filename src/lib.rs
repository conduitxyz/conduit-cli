// Proto does not derive `Eq` for the gRPC message types, which causes a warning from Clippy. The
// current suggestion is to explicitly allow the lint in the module that imports the protos.
// Read more: https://github.com/hyperium/tonic/issues/1056
#![allow(clippy::derive_partial_eq_without_eq)]

pub mod api;
pub mod handler;
pub mod opts;
pub mod utils;

// Generate the protobuf types.
pub mod types {
    include!(concat!(env!("OUT_DIR"), "/api.rs"));

    use std::str::FromStr;
    impl FromStr for DeploymentType {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "DEPLOYMENTTYPE_ANVIL" => DeploymentType::DeploymenttypeAnvil,
                "DEPLOYMENTTYPE_ERIGON" => DeploymentType::DeploymenttypeErigon,
                "DEPLOYMENTTYPE_GETH" => DeploymentType::DeploymenttypeGeth,
                _ => {
                    return Err(format!(
                        "unmatched variant. got {}, expected one of anvil, geth, erigon",
                        s
                    ))
                }
            })
        }
    }

    use serde::de::{self, Deserialize, Deserializer};
    pub fn deployment_type_from_str<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let s = match DeploymentType::from_str(s) {
            Ok(x) => x as i32,
            _ => return Err(de::Error::unknown_variant(s, &["anvil", "erigon", "geth"])),
        };
        Ok(s)
    }
}
