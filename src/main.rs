mod chrome;
mod chromiumoxide;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(e) = chrome::browse_wikipedia() {
        tracing::error!("browse wikipedia error ==> {e:?}");
    }

    Ok(())
}
