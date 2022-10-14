use clap::Parser;

#[derive(Clone, Debug, Parser)]
/// Parameters for auth'ing and connecting to the ExFac API.
pub struct ExFacOpts {
    // TODO: Remove default value.
    #[clap(env, long, short, default_value = "")]
    /// Your ExFac API key.
    pub api_key: String,
    #[clap(env, long, short, default_value = "https://api.exfac.xyz")]
    /// The URL pointing to the ExFac API.
    pub url: String,
    /// The default organization to scope our API requests to.
    #[clap(env, long, short, default_value = "")]
    pub organization: String,
}

impl ExFacOpts {
    // Returns the network slug.
    pub fn network(&self) -> String {
        format!("{}/v1/network", self.url)
    }

    pub fn job_template(&self) -> String {
        format!("{}/v1/job_template", self.url)
    }

    pub fn job(&self) -> String {
        format!("{}/v1/job", self.url)
    }

    // Returns the user slug.
    pub fn user(&self) -> String {
        format!("{}/v1/user", self.url)
    }
}
