use clap::Parser;
use uuid::Uuid;

use crate::api::{ExFac, Result};
use crate::types::{
    CreateJobTemplateRequest, CreateJobTemplateResponse, GetJobTemplatesRequest,
    GetJobTemplatesResponse,
};

use super::ClientError;

#[derive(Debug, Parser)]
/// Options for calling the /create endpoint on the API.
pub struct CreateOpts {
    /// The id of the job template we're creating or updating. By default we'll auto-generate
    /// an id for you. If you want to update an existing job template, you can pass its id in here.
    #[clap(env, short, long)]
    job_template: Option<Uuid>,

    /// The link to the public git repository to clone for this job template
    #[clap(short, long)]
    repository: Option<String>,

    /// The script to call that prepares the repository to run the command you want to run
    /// e.g. installing dependencies, compiling contracts
    #[clap(short, long, default_value = "")]
    prepare_command: String,

    /// The default command that will be used, if not overridden, when assigning jobs from this template
    #[clap(short = 'c', long, default_value = "")]
    default_command: String,

    /// The name of the job template
    #[clap(short, long, default_value = "")]
    pub name: String,

    /// The description for this job template
    #[clap(short, long, default_value = "")]
    description: String,

    /// The image url to be used in when this template is displayed in the UI
    #[clap(short, long, default_value = "https://app.exfac.xyz/logo492.png")]
    image_url: String,
}

impl ExFac {
    /// Returns a list of all the networks under the provided organization.
    pub async fn list_job_templates(&self) -> Result<GetJobTemplatesResponse> {
        let url = format!("{}/list", self.opts.job_template());
        self.post(
            url,
            GetJobTemplatesRequest {
                organization: self.opts.organization.to_string(),
            },
        )
        .await
    }

    /// Creates a new network for the provided options.
    pub async fn create_or_update_job_template(
        &self,
        opts: &CreateOpts,
    ) -> Result<CreateJobTemplateResponse> {
        const EMPTY: Uuid = Uuid::nil();
        let url = format!("{}/createOrUpdate", self.opts.job_template());

        let (job_template, repository) = match opts.job_template {
            Some(EMPTY) => return Err(ClientError::DefaultUuid),
            Some(inner) => (inner, opts.repository.to_owned().unwrap_or_default()),
            None => (
                Uuid::new_v4(),
                opts.repository
                    .to_owned()
                    .ok_or(ClientError::NoRepository)?,
            ),
        };

        self.post(
            url,
            CreateJobTemplateRequest {
                organization: self.opts.organization.to_string(),
                job_template: job_template.to_string(),
                repository,
                prepare_command: opts.prepare_command.to_owned(),
                default_command: opts.default_command.to_owned(),
                name: opts.name.to_owned(),
                description: opts.description.to_owned(),
                image_url: opts.image_url.to_owned(),
            },
        )
        .await
    }
}
