use clap::Parser;
use uuid::Uuid;

use crate::api::{ExFac, Result};
use crate::types::{
    CreateJobTemplateRequest, CreateJobTemplateResponse, GetJobTemplatesRequest,
    GetJobTemplatesResponse,
};

#[derive(Debug, Parser)]
/// Options for calling the /create endpoint on the API.
pub struct CreateOpts {
    /// The organization you want to create a network for.
    #[clap(env, short, long)]
    organization: Uuid,

    /// The id of the job template we're creating or updating. By default we'll auto-generate
    /// an id for you. If you want to update an existing job template, you can pass its id in here.
    #[clap(
        env,
        short,
        long,
        default_value = "00000000-0000-0000-0000-000000000000"
    )]
    job_template: Uuid,

    /// The link to the public git repository to clone for this job template
    #[clap(short, long)]
    repository: String,

    /// The script to call that prepares the repository to run the command you want to run
    /// e.g. installing dependencies, compiling contracts
    #[clap(short, long)]
    prepare_command: String,

    /// The default command that will be used, if not overridden, when assigning jobs from this template
    #[clap(short = 'c', long)]
    default_command: String,

    /// The name of the job template
    #[clap(short, long)]
    name: String,

    /// The description for this job template
    #[clap(short, long)]
    description: String,

    /// The image url to be used in when this template is displayed in the UI
    #[clap(short, long, default_value = "https://app.exfac.xyz/logo492.png")]
    image_url: String,
}

impl ExFac {
    /// Returns a list of all the networks under the provided organization.
    pub async fn list_job_templates(&self, organization: Uuid) -> Result<GetJobTemplatesResponse> {
        let url = format!("{}/list", self.opts.job_template());
        self.post(
            url,
            GetJobTemplatesRequest {
                organization: organization.to_string(),
            },
        )
        .await
    }

    /// Creates a new network for the provided options.
    pub async fn create_or_update_job_template(
        &self,
        opts: CreateOpts,
    ) -> Result<CreateJobTemplateResponse> {
        let url = format!("{}/createOrUpdate", self.opts.job_template());
        let mut job_template = opts.job_template.to_string();
        if job_template == "00000000-0000-0000-0000-000000000000" {
            job_template = Uuid::new_v4().to_string();
        }
        self.post(
            url,
            CreateJobTemplateRequest {
                organization: opts.organization.to_string(),
                job_template,
                repository: opts.repository,
                prepare_command: opts.prepare_command,
                default_command: opts.default_command,
                name: opts.name,
                description: opts.description,
                image_url: opts.image_url,
            },
        )
        .await
    }
}
