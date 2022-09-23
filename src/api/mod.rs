pub mod config;
use config::ExFacOpts;

pub mod network;

use eyre::Result;
use reqwest::Client;

use crate::types::LoadUserResponse;
use serde::{de::DeserializeOwned, Serialize};

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

    /// Returns the user's information
    pub async fn user(&self) -> Result<LoadUserResponse> {
        self.get(self.opts.user()).await
    }

    async fn get<R: DeserializeOwned>(&self, url: String) -> Result<R> {
        Ok(self
            .client
            .get(url)
            .bearer_auth(&self.opts.api_key)
            .send()
            .await?
            .json()
            .await?)
    }

    async fn post<T: Serialize, R: DeserializeOwned>(&self, url: String, req: T) -> Result<R> {
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
