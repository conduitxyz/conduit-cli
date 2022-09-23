use crate::api::ExFacOpts;
use clap::{Parser, Subcommand};

mod network;
mod user;

#[derive(Debug, Parser)]
#[clap(name = "exfac", version = crate::utils::VERSION_MESSAGE)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
    #[clap(flatten)]
    pub api: ExFacOpts,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "1-click deploy infrastructure for blockchains.",
    after_help = "Find more information in at: https://app.exfac.xyz"
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    #[clap(alias = "n")]
    Network(network::NetworkArgs),

    #[clap(visible_alias = "com", about = "Generate shell completions script.")]
    Completions {
        #[clap(arg_enum)]
        shell: clap_complete::Shell,
    },

    #[clap(alias = "u")]
    User(user::UserArgs),
}
