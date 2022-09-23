use vergen::{Config, ShaKind};

fn main() {
    let mut config = Config::default();
    // Change the SHA output to the short variant
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    vergen::vergen(config)
        .unwrap_or_else(|e| panic!("vergen crate failed to generate version information! {e}"));

    // Codegen the types from the protobufs
    // + add serde Serialize/Deserialize derives
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");

    // Rename the field attributes to match the API
    config.field_attribute("rpcURL", "#[serde(rename = \"rpcURL\")]");
    config.field_attribute("explorerURL", "#[serde(rename = \"explorerURL\")]");
    config.field_attribute("faucetURL", "#[serde(rename = \"faucetURL\")]");

    // Add logic for parsing the enums from strings.
    config.field_attribute(
        "Testnet.type",
        "#[serde(deserialize_with = \"crate::types::deployment_type_from_str\")]",
    );

    config
        .compile_protos(&["proto/api.proto"], &["proto/"])
        .unwrap();
}
