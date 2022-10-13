use clap::Parser;

use axum::{body::Body, http::Request, routing::post, Router};
use std::{net::SocketAddr, path::PathBuf};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::types::ApiKey;

pub const EXFAC_DIR_NAME: &str = ".exfac";
pub fn config_dir() -> PathBuf {
    dirs_next::home_dir()
        .map(|p| p.join(EXFAC_DIR_NAME))
        .expect("could not make config dir")
}

#[derive(Debug, Parser)]
pub struct Args {
    /// The port to which the callback should be done.
    #[clap(long, short, default_value = "8000")]
    port: u16,
}

impl Args {
    pub async fn run(self) -> eyre::Result<()> {
        println!(
            "Please visit https://app.exfac.xyz/cliauth?redirect_uri=http://localhost:{} to get your API Key",
            self.port
        );

        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        let app = Router::new().route(
            "/",
            post(|req: Request<Body>| async move {
                let (_, body) = req.into_parts();
                let bytes = hyper::body::to_bytes(body).await.unwrap();
                let resp = serde_json::from_slice(&bytes).unwrap();
                tx.send(resp).unwrap();
            }),
        );

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(signal(rx))
            .await
            .unwrap();

        Ok(())
    }
}

async fn signal(mut rx: UnboundedReceiver<ApiKey>) {
    let config_dir = config_dir().join("auth");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).expect("could not create the config directory");
    }

    let key_path = config_dir.join("api-key");
    let key = rx.recv().await.expect("no api key sent by server").key;
    std::fs::write(&key_path, key).expect("could not write api key to config");

    println!(
        "Logged in successfully! API Key stored at {}",
        key_path.display()
    );
    std::process::exit(0);
}
