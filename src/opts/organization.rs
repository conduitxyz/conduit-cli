use crate::config_dir;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct OrganizationArgs {
    #[clap(subcommand)]
    sub: Subcommands,
}


#[derive(Debug, Parser)]
pub struct SetArgs {
    /// The organiation we want to set as our default
    organization: String,
}

#[derive(Debug, Parser)]
pub struct GetArgs {
}

#[derive(Debug, Subcommand)]
/// Commands about setting and getting the current organization
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Gets the current organization
    Get(GetArgs),
    /// Sets the current organization to the provided input
    Set(SetArgs),
}

impl OrganizationArgs {
    pub async fn run(self) -> eyre::Result<()> {
        match self.sub {
            Subcommands::Get(_opts) => {
                let config_dir = config_dir();
                if !config_dir.exists() {
                    println!("No currently set organization. Use the command `conduit login` to set this automatically.")
                }
        
                let organization_path = config_dir.join("organization");
                let organization = std::fs::read_to_string(&organization_path).expect("could not read organization config");
        
                println!(
                    "Current Organization is {}",
                    organization,
                );
            },
            Subcommands::Set(opts) => {
                let config_dir = config_dir();
                if !config_dir.exists() {
                    std::fs::create_dir_all(&config_dir).expect("could not create the config directory");
                }
        
                let organization_path = config_dir.join("organization");
                std::fs::write(&organization_path, opts.organization).expect("could not write organization to config");
        
                println!(
                    "Organization successfully set at {}",
                    organization_path.display(),
                );
            },
        }
        Ok(())
    }
}