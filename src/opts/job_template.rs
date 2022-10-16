use clap::{Parser, Subcommand};
use std::fmt::Write;
use uuid::Uuid;

use crate::api::{job_template::CreateOpts, ClientError, ExFac};

#[derive(Debug, Parser)]
// TODO: Should the user set a default organization client side
// in some config when they auth, instead of having to specify it all the time?
// And maybe make it part of ExFac config?
pub struct Args {
    #[clap(subcommand)]
    sub: Subcommands,
}

#[derive(Debug, Subcommand)]
/// Commands about interacting with the various job templates you have spinned up
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Lists all job templates.
    List,

    /// Creates or updates a job template.
    CreateOrUpdate(CreateOpts),
}

impl Args {
    pub async fn run(self, exfac: ExFac) -> eyre::Result<()> {
        match self.sub {
            Subcommands::List => {
                let resp = exfac.list_job_templates().await?;
                for job_template in resp.templates {
                    println!("Name: {}", &job_template.name);
                    let mut job_template = serde_json::to_value(&job_template)?;
                    let obj = job_template.as_object_mut().unwrap();
                    obj.remove("name");

                    let table = to_table(job_template);
                    println!("{}", table);
                }
            }
            Subcommands::CreateOrUpdate(opts) => {
                match exfac.create_or_update_job_template(&opts).await {
                    Ok(resp) => println!(
                        "Job template {} created.\nResponse: {}",
                        opts.name,
                        serde_json::to_string_pretty(&resp)?
                    ),
                    Err(ClientError::EmptyResponse) => println!(
                        "Job Template with name \"{}\" already exists. Try changing the name.",
                        opts.name
                    ),
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
