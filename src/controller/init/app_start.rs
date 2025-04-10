use crate::controller::init::config::init_config;
use crate::controller::init::log::init_log;
use crate::controller::init::url::init_url;
use crate::controller::init::web::init_web;
use crate::model::clickhouse::proxy::init_clickhouse;
use crate::model::mysql::proxy::init_mysql;
use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "CabinetApp")]
#[command(author = "ZHangQL")]
#[command(version = "1.0")]
#[command(about="机柜功耗管理APP",long_about=None)]
struct Cabinet {
    // value_name  -c, --config <FILE>  Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

/// init the config path
pub fn init() -> Result<PathBuf> {
    let cli = Cabinet::parse();
    match cli.config {
        Some(path) => Ok(path),
        None => Err(anyhow!("must input config path")),
    }
}

/// run the app
pub async fn run(path: PathBuf) -> Result<()> {
    let setting = init_config(path)?;
    init_log(&setting.log)?;

    init_url(setting.url).await;

    init_mysql(setting.mysql).await?;

    init_clickhouse(setting.clickhouse).await?;

    init_web(setting.web).await?;

    Ok(())
}
