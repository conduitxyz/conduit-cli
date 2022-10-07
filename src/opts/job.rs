use crate::api::{job::AssignOpts, ExFac};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    sub: Subcommands,
}

#[derive(Debug, Subcommand)]
/// Commands about interacting with the various jobs
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Creates or updates a job
    Assign(AssignOpts),
}

impl Args {
    pub async fn run(self, exfac: ExFac) -> eyre::Result<()> {
        match self.sub {
            Subcommands::Assign(opts) => {
                let resp = exfac.assign(opts).await?;
                println!("{}", serde_json::to_string(&resp)?);
            }
        }
        Ok(())
    }
}
