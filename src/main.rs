use anyhow::Result;
use cabinet::controller::init::app_start::{init, run};

#[tokio::main]
async fn main() -> Result<()> {
    run(init()?).await?;
    Ok(())
}
