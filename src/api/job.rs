use clap::Parser;
use uuid::Uuid;

use crate::api::{ExFac, Result};
use crate::types::{
    CreateJobRequest, CreateJobResponse, EnvironmentVariable, GetAllJobRunsRequest, TriggerOnDemandJobRequest,
    GetAllJobRunsResponse, GetJobRunStatusRequest, GetJobRunStatusResponse, TriggerOnDemandJobResponse
};

#[derive(Debug, Parser)]
/// Options for calling the /create endpoint on the API.
pub struct AssignOpts {
    /// The organization you want to create a job for.
    #[clap(env, short, long)]
    organization: Uuid,

    /// The id of the job template we're using as a base
    #[clap(env, long)]
    template: Uuid,

    /// The id of the network to assign this job to
    #[clap(env, long)]
    network: Uuid,

    /// The command that will be executed in the job. Will use the
    /// job's default command otherwise.
    #[clap(short, long)]
    execute_command: Option<String>,

    /// The name of the job
    #[clap(short, long, default_value = "")]
    name: String,

    /// The description for this job
    #[clap(short, long, default_value = "")]
    description: String,

    /// OnStart, OnEnd, OnDemand. Default: OnDemand
    #[clap(short, long, default_value = "2")]
    r#type: i32,

    /// A semi-colon separated list of key-value pairs, e.g "--env key:val key2:val2"
    /// which get passed on to the called script as envirnoment variables (case sensitive)
    #[clap(long, num_args(0..))]
    env: Vec<EnvironmentVariable>,
}

#[derive(Debug, Parser)]
/// Options for calling the /runs/all endpoint on the API.
pub struct ListOpts {
    /// The organization you want to list jobs for
    #[clap(env, short, long)]
    organization: Uuid,
    /// The network you want to list jobs for
    #[clap(env, long)]
    network: Uuid,
}

#[derive(Debug, Parser)]
/// Options for calling the /runs/status endpoint on the API.
pub struct StatusOpts {
    /// The organization you want to list jobs for
    #[clap(env, short, long)]
    organization: Uuid,
    /// The job template uuid
    #[clap(env, long)]
    job: Uuid,
    /// The run uuid of the job instance
    #[clap(env, long)]
    run: Uuid,
}

#[derive(Debug, Parser)]
/// Options for calling the /triggerOnDemand endpoint
pub struct TriggerOpts {
    /// The organization you want to list jobs for
    #[clap(env, short, long)]
    organization: Uuid,
    /// The job template uuid
    #[clap(env, long)]
    job: Uuid,
}

// TODO: Investigate whether we want to split in pure create/update apis.
impl ExFac {
    /// Assigns the provided job template to a live network, creating a job.
    #[tracing::instrument(skip(self, opts))]
    pub async fn assign(&self, opts: AssignOpts) -> Result<CreateJobResponse> {
        tracing::debug!(?opts, "assigning job");
        
        let url = format!("{}/create", self.opts.job());
        self.post(
            url,
            CreateJobRequest {
                organization: opts.organization.to_string(),
                job_template: opts.template.to_string(),
                job: Uuid::new_v4().to_string(),
                testnet: opts.network.to_string(),
                execute_command: opts.execute_command.clone().unwrap_or_default(),
                name: opts.name,
                description: opts.description,
                r#type: opts.r#type,
                schedule: "".to_owned(),
                variables: opts.env,
                use_default_command: opts.execute_command.clone().unwrap_or_default() == "",
            },
        )
        .await
    }

    /// Lists all the jobs in the network
    #[tracing::instrument(skip(self, opts))]
    pub async fn list(&self, opts: ListOpts) -> Result<GetAllJobRunsResponse> {
        tracing::debug!(?opts, "getting all jobs");
        let url = format!("{}/runs/all", self.opts.job());
        self.post(
            url,
            GetAllJobRunsRequest {
                organization: opts.organization.to_string(),
                testnet: opts.network.to_string(),
            },
        )
        .await
    }

    /// Gets the status of the specified job
    #[tracing::instrument(skip(self, opts))]
    pub async fn status(&self, opts: StatusOpts) -> Result<GetJobRunStatusResponse> {
        tracing::debug!(?opts, "getting job");
        let url = format!("{}/runs/status", self.opts.job());
        self.post(
            url,
            GetJobRunStatusRequest {
                organization: opts.organization.to_string(),
                job: opts.job.to_string(),
                run: opts.run.to_string(),
            },
        )
        .await
    }

    /// Gets the status of the specified job
    #[tracing::instrument(skip(self, opts))]
    pub async fn trigger(&self, opts: TriggerOpts) -> Result<TriggerOnDemandJobResponse> {
        tracing::debug!(?opts, "triggering job");
        let url = format!("{}/triggerOnDemand", self.opts.job());
        self.post(
            url,
            TriggerOnDemandJobRequest {
                organization: opts.organization.to_string(),
                job: opts.job.to_string(),
            },
        )
        .await
    }
}