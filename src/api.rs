use clap::Parser;
use eyre::Result;
use reqwest::Client;
use uuid::Uuid;

use crate::types::{ListTestnetsRequest, ListTestnetsResponse};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone, Debug, Parser)]
/// Parameters for auth'ing and connecting to the ExFac API.
pub struct ExFacOpts {
    // TODO: Remove default value.
    #[clap(long, short, default_value = "5580b8eb-0d8f-482e-936b-335f2ff6332d")]
    /// Your ExFac API key.
    pub api_key: String,
    #[clap(long, short, default_value = "http://localhost:8080")]
    /// The URL pointing to the ExFac API.
    pub url: String,
}

impl ExFacOpts {
    // Returns the network slug.
    fn network(&self) -> String {
        format!("{}/v1/testnet", self.url)
    }
}

/// The ExFac REST API client.
#[derive(Clone, Debug)]
pub struct ExFac {
    client: Client,
    opts: ExFacOpts,
}

impl ExFac {
    /// Instantiates a client from the provided opts.
    pub fn new(opts: ExFacOpts) -> Self {
        Self {
            client: Client::new(),
            opts,
        }
    }

    /// Returns a list of all the testnets under the provided organization.
    pub async fn list(&self, organization: Uuid) -> Result<ListTestnetsResponse> {
        let url = format!("{}/list", self.opts.network());
        self.request(
            url,
            ListTestnetsRequest {
                organization: organization.to_string(),
            },
        )
        .await
    }

    async fn request<T: Serialize, R: DeserializeOwned>(&self, url: String, req: T) -> Result<R> {
        let res = self
            .client
            .post(url)
            .bearer_auth(&self.opts.api_key)
            .json(&req)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }
}
