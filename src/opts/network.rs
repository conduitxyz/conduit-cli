use clap::{Parser, Subcommand};
use uuid::Uuid;

use crate::api::ExFac;

#[derive(Debug, Parser)]
pub struct NetworkArgs {
    #[clap(subcommand)]
    sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(about = "Commands about interacting with the various networks you have spinned up")]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Lists all networks under the provided organization.
    List {
        #[clap(short, long)]
        /// The organization you want to list networks for.
        organization: Uuid,
    },
}

impl NetworkArgs {
    pub async fn run(&self, exfac: ExFac) -> eyre::Result<()> {
        match self.sub {
            Subcommands::List { organization } => {
                let resp = exfac.list(organization).await?;
                println!("{}", serde_json::to_string(&resp)?);
            }
        }
        Ok(())
    }
}
