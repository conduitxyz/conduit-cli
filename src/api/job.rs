use clap::Parser;
use uuid::Uuid;

use crate::api::ExFac;
use crate::types::{CreateJobRequest, CreateJobResponse, EnvironmentVariable};

use eyre::Result;

#[derive(Debug, Parser)]
/// Options for calling the /create endpoint on the API.
pub struct AssignOpts {
    /// The organization you want to create a job for.
    #[clap(short, long)]
    organization: Uuid,

    /// The id of the job template we're using as a base
    #[clap(long)]
    template: Uuid,

    /// The id of the job we're going to create. By default we'll auto-generate an id
    /// for you, but you can also manually specify this to update an existing job.
    #[clap(short, long, default_value = "00000000-0000-0000-0000-000000000000")]
    job: Uuid,

    /// The id of the network to assign this job to
    #[clap(long)]
    network: Uuid,

    /// The command that will be executed in the job. Will use the
    /// job's default command otherwise.
    #[clap(short, long)]
    execute_command: Option<String>,

    /// The name of the job
    #[clap(short, long)]
    name: String,

    /// The description for this job
    #[clap(short, long, default_value = "")]
    description: String,

    #[clap(short, long, default_value = "2")]
    r#type: i32,

    #[clap(long, num_args(0..))]
    env: Vec<EnvironmentVariable>,
}

// TODO: Investigate whether we want to split in pure create/update apis.
impl ExFac {
    /// Creates a new network for the provided options.
    #[tracing::instrument(skip(self, opts))]
    pub async fn assign(&self, opts: AssignOpts) -> Result<CreateJobResponse> {
        tracing::debug!(?opts, "assigning job");
        let url = format!("{}/create", self.opts.job());
        let use_default_command = opts.execute_command.is_none();
        self.post(
            url,
            CreateJobRequest {
                organization: opts.organization.to_string(),
                job_template: opts.template.to_string(),
                job: opts.job.to_string(),
                testnet: opts.network.to_string(),
                execute_command: opts.execute_command.unwrap_or_default(),
                name: opts.name,
                description: opts.description,
                r#type: opts.r#type,
                schedule: "".to_owned(),
                variables: opts.env,
                use_default_command,
            },
        )
        .await
    }
}
