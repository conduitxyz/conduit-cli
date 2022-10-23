use crate::api::{
    job::{AssignOpts, ListOpts, StatusOpts, TriggerOpts},
    Conduit,
};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    sub: Subcommands,
}

#[derive(Debug, Subcommand)]
/// Commands about interacting with the various Conduit
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Creates or updates a job
    Assign(AssignOpts),
    /// Lists all historical Conduit for the specified org/network
    List(ListOpts),
    /// Gets the status of a specified job (can be any of the historical ones, or any ones running
    /// at the moment)
    Status(StatusOpts),
    /// Triggers the given job immediately.
    Trigger(TriggerOpts),
}

impl Args {
    pub async fn run(self, conduit: Conduit) -> eyre::Result<()> {
        match self.sub {
            Subcommands::Assign(opts) => {
                let resp = conduit.assign(opts).await?;
                println!("{}", serde_json::to_string(&resp)?);
            }
            Subcommands::List(opts) => {
                let resp = conduit.list(opts).await?;
                println!("{}", serde_json::to_string(&resp)?);
            }
            Subcommands::Status(opts) => {
                let resp = conduit.status(opts).await?;
                println!("{}", serde_json::to_string(&resp)?);
            }
            Subcommands::Trigger(opts) => {
                conduit.trigger(opts).await?;
                println!("Triggered job!");
            }
        }
        Ok(())
    }
}
