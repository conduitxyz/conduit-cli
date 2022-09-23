// CLI
use clap::{IntoApp, Parser};
use clap_complete::generate;

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

    let opts = Opts::parse();
    let exfac = ExFac::new(opts.api);

    match opts.sub {
        Subcommands::Completions { shell } => {
            generate(shell, &mut Opts::command(), "exfac", &mut std::io::stdout())
        }
        Subcommands::Network(args) => args.run(exfac).await?,
        Subcommands::User(args) => args.run(exfac).await?,
    }

    Ok(())
}
