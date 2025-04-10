use crate::controller::init::config::Web;
use anyhow::Result;
use chrono::Local;

use crate::controller::init::route::route;
use tokio::net::TcpListener;
use tokio::signal;

/// init web by config
pub async fn init_web(conf: Web) -> Result<()> {
    println!(
        "TIME {} WEB IS LISTENING: {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        conf.host
    );
    axum::serve(TcpListener::bind(conf.host).await?, route())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    println!("Signal received, starting graceful shutdown");
}
