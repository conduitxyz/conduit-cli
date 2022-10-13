// Proto does not derive `Eq` for the gRPC message types, which causes a warning from Clippy. The
// current suggestion is to explicitly allow the lint in the module that imports the protos.
// Read more: https://github.com/hyperium/tonic/issues/1056
#![allow(clippy::derive_partial_eq_without_eq)]

pub mod api;
pub mod handler;
pub mod opts;
pub mod utils;

pub const EXFAC_DIR_NAME: &str = ".exfac";
pub fn config_dir() -> std::path::PathBuf {
    dirs_next::home_dir()
        .expect("could not make config dir")
        .join(EXFAC_DIR_NAME)
        .join("auth")
}

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

    // Parses EnvVars as a semi-colon separated list of key-value pairs, e.g "key:val"
    impl FromStr for EnvironmentVariable {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut it = s.split(':');
            let name = it
                .next()
                .ok_or_else(|| format!("No Key found in env var: {s}"))?
                .to_string();
            if name.is_empty() {
                return Err(format!("No Key found in env var: {s}"));
            }

            let value = it
                .next()
                .ok_or_else(|| format!("No Value found in env var: {s}"))?
                .to_string();
            if value.is_empty() {
                return Err(format!("No Value found in env var: {s}"));
            }

            Ok(Self { name, value })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use types::EnvironmentVariable;

    #[test]
    fn parse_envvar() {
        assert_eq!(
            EnvironmentVariable::from_str("key:val").unwrap(),
            EnvironmentVariable {
                name: "key".to_string(),
                value: "val".to_string()
            }
        );
        assert_eq!(
            EnvironmentVariable::from_str("").unwrap_err(),
            "No Key found in env var: ",
        );
        assert_eq!(
            EnvironmentVariable::from_str("key:").unwrap_err(),
            "No Value found in env var: key:",
        );
    }
}
