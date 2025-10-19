use anyhow::Result;
use chromiumoxide::Browser;
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::{sleep, timeout};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    loop {
        match Browser::connect("ws://0.0.0.0:9222/").await {
            Ok((browser, mut handler)) => {
                tracing::info!("✅ Connected to Lightpanda");

                // handler 必须持续运行，否则所有 CDP 命令都会失败
                tokio::spawn(async move {
                    while let Some(evt) = handler.next().await {
                        match evt {
                            Ok(ev) => {
                                tracing::trace!("📡 CDP Event: {:?}", ev);
                            }
                            Err(e) => {
                                tracing::error!("💥 Browser handler error: {e}");
                                break;
                            }
                        }
                    }
                    tracing::warn!("⚠️ Browser handler exited");
                });

                // 实际执行任务
                if let Err(e) = run(&browser).await {
                    tracing::error!("💥 Browser session error: {e:?}");
                }

                tracing::warn!("🔌 Disconnected, will retry...");
            }

            Err(e) => {
                tracing::error!("⚠️ Connect error: {e:?}");
            }
        }

        tracing::info!("⏳ Waiting 3s before reconnect...");
        sleep(Duration::from_secs(3)).await;
    }
}

async fn run(browser: &Browser) -> Result<()> {
    // 设置超时，避免 hang 住
    let page = timeout(Duration::from_secs(10), browser.new_page("https://en.wikipedia.org"))
        .await
        .map_err(|_| anyhow::anyhow!("⏰ new_page timeout"))??;

    tracing::info!("🌐 Page opened");

    // 等待 input 元素
    let input = timeout(Duration::from_secs(5), page.find_element("input#searchInput"))
        .await
        .map_err(|_| anyhow::anyhow!("⏰ wait input timeout"))??;

    input.click().await?;
    input.type_str("Rust programming language").await?;
    input.press_key("Enter").await?;

    tracing::info!("🔍 Searching...");

    let nav = timeout(Duration::from_secs(10), page.wait_for_navigation())
        .await
        .map_err(|_| anyhow::anyhow!("⏰ navigation timeout"))??;

    let html = nav.content().await?;
    tracing::info!("✅ Page loaded, length: {}", html.len());

    Ok(())
}
