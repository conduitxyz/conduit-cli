use clap::Parser;
use uuid::Uuid;

use crate::api::ExFac;
use crate::types::{
    CreateJobRequest, CreateJobResponse, TriggerOnDemandJobRequest, EnvironmentVariable
};

use eyre::Result;

#[derive(Debug, Parser)]
/// Options for calling the /create endpoint on the API.
pub struct CreateOpts {
    /// The organization you want to create a job for.
    #[clap(short, long)]
    organization: Uuid,

    /// The id of the job template we're using as a base
    #[clap(short, long)]
    job_template: Uuid,

    /// The id of the job we're going to create. By default we'll auto-generate an id
    /// for you, but you can also manually specify this to update an existing job.
    #[clap(short, long, default_value = "00000000-0000-0000-0000-000000000000")]
    job: Uuid,

    /// The id of the network to assign this job to
    #[clap(short, long)]
    network: Uuid,

    /// The command that will be executed in the job. Will be overriden if
    /// use-default-command is set to true
    #[clap(short, long, required_unless = "use_default_command")]
    execute_command: String,

    /// Whether to use the default command from the job template specified
    #[clap(short = 'c', long)]
    use_default_command: bool,

    /// The name of the job
    #[clap(short, long)]
    name: String,

    /// The description for this job
    #[clap(short, long)]
    description: String,

    /// The image url to be used in when this template is displayed in the UI
    #[clap(short, long, default_value = "2")]
    job_type: i32,

    #[clap(short, long)]
    environment_variables: Vec<EnvironmentVariable>,

}

impl ExFac {
    /// Creates a new network for the provided options.
    pub async fn create_or_update_job(&self, opts: CreateOpts) -> Result<CreateJobResponse> {
        let url = format!("{}/create", self.opts.job());
        let mut job = opts.job.to_string();
        if job == "00000000-0000-0000-0000-000000000000" {
            job = Uuid::new_v4().to_string();
        }
        self.post(
            url,
            CreateJobRequest {
                organization: opts.organization.to_string(),
                job_template: opts.job_template.to_string(),
                job: job,
                testnet: opts.network.to_string(),
                execute_command: opts.execute_command,
                //use_default_command: opts.use_default_command,
                name: opts.name,
                description: opts.description,
                r#type: opts.job_type,
                schedule: "".to_owned(),
                variables: 
            },
        )
        .await
    }
}
