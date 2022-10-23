use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use yansi::Paint;

/// The version message for the current program, like
/// `forge 0.1.0 (f01b232bc 2022-01-22T23:28:39.493201+00:00)`
pub(crate) const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA_SHORT"),
    " ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")"
);

/// Disables terminal colours if either:
/// - Running windows and the terminal does not support colour codes.
/// - Colour has been disabled by some environment variable.
/// - We are running inside a test
pub fn enable_paint() {
    let is_windows = cfg!(windows) && !Paint::enable_windows_ascii();
    let env_colour_disabled = std::env::var("NO_COLOR").is_ok();
    if is_windows || env_colour_disabled {
        Paint::disable();
    }
}

pub fn subscriber() {
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer())
        .init()
}
