// CLI
use clap::{IntoApp, Parser};
use clap_complete::generate;

// Logging
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

use crate::utils;

fn main() -> eyre::Result<()> {
    dotenv::dotenv()?;
    handler::install()?;
    utils::subscriber();
    utils::enable_paint();

    let opts = Opts::parse();
    match opts.sub {}

    Ok(())
}
