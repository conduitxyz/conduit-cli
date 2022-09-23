use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRequest {
    pub organization: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    testnets: Vec<ListResponseInner>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListResponseInner {
    testnet: String,
    name: String,
    #[serde(rename = "rpcURL")]
    rpc_url: String,
    #[serde(rename = "explorerURL")]
    explorer_url: String,
    #[serde(rename = "faucetURL")]
    faucet_url: String,
    #[serde(rename = "type")]
    testnet_type: String,
}
