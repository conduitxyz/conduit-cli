use clap::Parser;
use uuid::Uuid;

use crate::api::ExFac;
use crate::types::{
    create_testnet_options::Mining, CreateTestnetOptions, CreateTestnetRequest,
    CreateTestnetResponse, DeploymentType, CreateJobRequest
};
use crate::types::{
    DeleteTestnetRequest, DeleteTestnetResponse, ListTestnetsRequest, ListTestnetsResponse,
};

use eyre::Result;

#[derive(Debug, Parser)]
/// Options for calling the /create endpoint on the API.
pub struct CreateOpts {
    /// The organization you want to create a network for.
    #[clap(short, long)]
    organization: Uuid,

    /// The name of the network you are creating.
    #[clap(short, long)]
    name: String,

    /// The chain-id of the network
    #[clap(short, long, default_value = "888")]
    chain_id: usize,

    /// The memory you want to allocate (in MB)
    #[clap(short, long, default_value = "20000")]
    memory: usize,

    /// The memory you want to allocate (in MB)
    #[clap(long, default_value = "1")]
    cpu: usize,

    /// URL to remote network to fork off. ONLY available in Anvil.
    #[clap(short, long, default_value = "")]
    fork_url: String,

    /// Block number to fork off. ONLY available in Anvil.
    #[clap(long, default_value = "0")]
    fork_block: usize,

    /// Choose your deployment type.
    #[clap(long, default_value = "DEPLOYMENTTYPE_ANVIL")]
    deployment_type: DeploymentType,

    /// Optionally set the block time. If not provided, will insta-mine.
    #[clap(long)]
    block_time: Option<usize>,
}

#[derive(Debug, Parser)]
/// Options for calling the /delete endpoint on the API.
pub struct DeleteOpts {
    /// The organization you want to delete a network for.
    #[clap(short, long)]
    pub organization: Uuid,

    /// The name of the network you are deleting.
    #[clap(short, long)]
    pub name: Uuid,
}

impl ExFac {
    /// Returns a list of all the networks under the provided organization.
    pub async fn list_networks(&self, organization: Uuid) -> Result<ListTestnetsResponse> {
        let url = format!("{}/list", self.opts.network());
        self.post(
            url,
            ListTestnetsRequest {
                organization: organization.to_string(),
            },
        )
        .await
    }

    /// Creates a new network for the provided options.
    pub async fn create_network(&self, opts: CreateOpts) -> Result<CreateTestnetResponse> {
        let url = format!("{}/create", self.opts.network());
        self.post(
            url,
            CreateTestnetRequest {
                organization: opts.organization.to_string(),
                testnet: Uuid::new_v4().to_string(),
                opts: Some(CreateTestnetOptions {
                    name: opts.name,
                    fork_url: opts.fork_url,
                    fork_block_number: opts.fork_block as i64,
                    genesis_json: "".to_string(),
                    gas_limit: 30_000_000,
                    block_base_fee_per_gas: 1_000_000_000,
                    gas_price: 0,
                    // TODO: Why are these ints/floats etc?
                    chain_id: opts.chain_id as i32,
                    cpu_requests: opts.cpu as f64,
                    memory_requests_mb: opts.memory as i32,
                    r#type: opts.deployment_type as i32,
                    // TODO: Why do we just not make `mining` a optional u64
                    // in proto?
                    mining: opts.block_time.map(|x| Mining::BlockTime(x as i32)),
                }),
                jobs: Vec::<CreateJobRequest>::new(),
            },
        )
        .await
    }

    /// Deletes a network of your choice.
    pub async fn delete_network(&self, organization: Uuid, name: Uuid) -> Result<DeleteTestnetResponse> {
        let url = format!("{}/delete", self.opts.network());
        self.post(
            url,
            DeleteTestnetRequest {
                organization: organization.to_string(),
                testnet: name.to_string(),
            },
        )
        .await
    }
}
