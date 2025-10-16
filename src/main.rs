use chromiumoxide::Browser;
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    loop {
        match Browser::connect("ws://127.0.0.1:9222/").await {
            Ok((browser, mut handler)) => {
                tracing::info!("✅ Connected to Lightpanda");
                tokio::spawn(async move { while handler.next().await.is_some() {} });

                if let Err(e) = run(&browser).await {
                    tracing::error!("💥 Browser session error: {e}");
                }

                // 连接断开
                tracing::info!("🔌 Disconnected. Reconnecting...");
            }
            Err(e) => tracing::error!("⚠️ Connect error: {e}"),
        }

        tracing::info!("try connect. wait 3s...");
        sleep(Duration::from_secs(3)).await;
    }
}

async fn run(browser: &Browser) -> anyhow::Result<()> {
    let page = browser.new_page("https://en.wikipedia.org").await?;
    page.find_element("input#searchInput")
        .await?
        .click()
        .await?
        .type_str("Rust programming language")
        .await?
        .press_key("Enter")
        .await?;

    let html = page.wait_for_navigation().await?.content().await?;
    tracing::info!("Page length: {}", html.len());
    Ok(())
}
