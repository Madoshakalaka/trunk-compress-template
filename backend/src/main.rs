
use std::net::SocketAddr;

use anyhow::Result;
use axum::Router;
mod yew;

#[derive(Clone)]
pub struct AppState;

async fn run() -> Result<()> {
    #[cfg(feature = "journald")]
    {
        use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

        tracing_subscriber::registry()
            .with(tracing_journald::layer().unwrap())
            .init();
    }
    #[cfg(feature = "env-filter")]
    {
        use tracing_subscriber::prelude::*;

        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    };

    let app = Router::new().fallback_service(yew::make_service(()));

    let port = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(1001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(_) => {}
        Err(e) => {
            tracing::error!("{}", e)
        }
    }
}
