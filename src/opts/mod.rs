use crate::api::config::ExFacOpts;
use clap::{Parser, Subcommand};

mod job;
mod job_template;
mod login;
mod network;
mod user;
mod organization;

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
        #[clap(value_enum)]
        shell: clap_complete::Shell,
    },

    #[clap(alias = "u")]
    User(user::Args),

    #[clap(alias = "jt")]
    JobTemplate(job_template::Args),

    #[clap(alias = "j")]
    Job(job::Args),

    #[clap(alias = "l")]
    Login(login::Args),

    #[clap(alias = "o")]
    Organization(organization::OrganizationArgs),
}
