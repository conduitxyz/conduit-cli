// CLI
use clap::{CommandFactory, Parser};
use clap_complete::generate;

/*
use std::fs::File
use dirs
use config::Config
*/

use exfac::{
    api::ExFac,
    handler,
    opts::{Opts, Subcommands},
    utils,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // dotenv::dotenv()?;
    handler::install()?;
    utils::subscriber();
    utils::enable_paint();

    let mut opts = Opts::parse();
    if opts.api.api_key.is_empty() && !matches!(opts.sub, Subcommands::Login(_)) {
        opts.api.api_key = match std::fs::read_to_string(exfac::config_dir().join("api-key")) {
            Ok(key) => key,
            Err(_) => eyre::bail!("No API Key found. Either login via `conduit login` or provide `--api-key` (or set via env var `API_KEY`)")
        };
        opts.api.organization = match std::fs::read_to_string(exfac::config_dir().join("organization")) {
            Ok(key) => key,
            Err(_) => eyre::bail!("No Organization found. Either login via `conduit login` or provide `--organization` (or set via env var `ORGANIZATION`)")
        };
    }
    tracing::debug!(?opts);
    let exfac = ExFac::new(opts.api);

    match opts.sub {
        Subcommands::Completions { shell } => {
            generate(shell, &mut Opts::command(), "exfac", &mut std::io::stdout())
        }
        Subcommands::Network(args) => args.run(exfac).await?,
        Subcommands::User(args) => args.run(exfac).await?,
        Subcommands::JobTemplate(args) => args.run(exfac).await?,
        Subcommands::Job(args) => args.run(exfac).await?,
        Subcommands::Login(args) => args.run().await?,
        Subcommands::Organization(args) => args.run().await?,
    }

    Ok(())
}
