use clap::{Parser, Subcommand};
use std::fmt::Write;

use crate::api::{
    network::{CreateOpts, DeleteOpts},
    ClientError, ExFac,
};

#[derive(Debug, Parser)]
// TODO: Should the user set a default organization client side
// in some config when they auth, instead of having to specify it all the time?
// And maybe make it part of ExFac config?
pub struct NetworkArgs {
    #[clap(subcommand)]
    sub: Subcommands,
}

#[derive(Debug, Subcommand)]
/// Commands about interacting with the various networks you have spinned up
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Lists all networks.
    List,

    /// Creates a new network.
    Create(CreateOpts),

    /// Deletes a network.
    Delete(DeleteOpts),
}

impl NetworkArgs {
    pub async fn run(self, exfac: ExFac) -> eyre::Result<()> {
        match self.sub {
            Subcommands::List => {
                let resp = exfac.list_networks().await?;
                for network in resp.networks {
                    println!("Name: {}", &network.name);
                    let mut network = serde_json::to_value(&network)?;
                    let obj = network.as_object_mut().unwrap();
                    obj.remove("name");

                    let table = to_table(network);
                    println!("{}", table);
                }
            }
            Subcommands::Create(opts) => match exfac.create_network(&opts).await {
                Ok(resp) => println!(
                    "Network {} created\nResponse: {}",
                    opts.name,
                    serde_json::to_string_pretty(&resp)?
                ),
                Err(ClientError::EmptyResponse) => println!(
                    "Network with name {} already exists. Try changing the name.",
                    opts.name
                ),
                Err(err) => eyre::bail!(err),
            },
            Subcommands::Delete(opts) => {
                match exfac.delete_network(opts.network).await {
                    Ok(_) => println!("Network {} deleted", opts.network),
                    // Err(ClientError::EmptyResponse) => println!("Network not found. Did you already delete it? Is there a typo in your network id?"),
                    Err(err) => eyre::bail!(err),
                }
            }
        }
        Ok(())
    }
}

/// Given a k/v serde object, it pretty prints its keys and values as a table.
pub fn to_table(value: serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s,
        serde_json::Value::Object(map) => {
            let mut s = String::new();
            for (k, v) in map.iter() {
                writeln!(&mut s, "{: <20} {}", k, v).expect("could not write k/v to table");
            }
            s
        }
        _ => "".to_owned(),
    }
}
