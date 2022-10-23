use crate::api::Conduit;
use clap::Parser;

#[derive(Debug, Parser)]
/// Get information about your current session.
pub struct Args;

impl Args {
    pub async fn run(&self, conduit: Conduit) -> eyre::Result<()> {
        let resp = conduit.user().await?;
        println!("{}", serde_json::to_string(&resp)?);
        Ok(())
    }
}
