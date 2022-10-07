pub mod config;
use config::ExFacOpts;

pub mod job;
pub mod job_template;
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
        let res = self
            .client
            .get(url)
            .header(
                "Authorization".to_owned(),
                format!("Token {}", &self.opts.api_key),
            )
            .header(
                "User-Agent".to_owned(),
                format!("conduit-cli/{}", env!("VERGEN_GIT_SHA_SHORT")),
            )
            .send()
            .await?;
        let body = res.bytes().await?;

        let res = serde_json::from_slice(&body).map_err(|err| ClientError::SerdeJson {
            err,
            text: String::from_utf8_lossy(&body).to_string(),
        })?;

        Ok(res)
    }

    async fn post<T: Serialize + std::fmt::Debug, R: DeserializeOwned>(
        &self,
        url: String,
        req: T,
    ) -> Result<R> {
        let res = self
            .client
            .post(url)
            .header(
                "Authorization".to_owned(),
                format!("Token {}", &self.opts.api_key),
            )
            .header(
                "User-Agent".to_owned(),
                format!("conduit-cli/{}", env!("VERGEN_GIT_SHA_SHORT")),
            )
            .json(&req)
            .send()
            .await?;

        let body = res.bytes().await?;

        let res = serde_json::from_slice(&body).map_err(|err| ClientError::SerdeJson {
            err,
            text: String::from_utf8_lossy(&body).to_string(),
        })?;

        Ok(res)
    }
}

#[derive(thiserror::Error, Debug)]
/// Error thrown when sending an HTTP request
pub enum ClientError {
    /// Thrown if the request failed
    #[error("Deserialization Error: {err}. Response: {text}")]
    /// Serde JSON Error
    SerdeJson {
        err: serde_json::Error,
        text: String,
    },
}
