use crate::api::ExFac;
use clap::Parser;

#[derive(Debug, Parser)]
/// Get information about your current session.
pub struct UserArgs;

impl UserArgs {
    pub async fn run(&self, exfac: ExFac) -> eyre::Result<()> {
        let resp = exfac.user().await?;
        println!("{}", serde_json::to_string(&resp)?);
        Ok(())
    }
}
