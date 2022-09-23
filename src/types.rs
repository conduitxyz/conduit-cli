

#[derive(Serialize, Deserialize)]
pub struct ListResponse {
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
