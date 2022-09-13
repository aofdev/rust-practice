use color_eyre::Report;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use std::{future::Future, time::Duration};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn fetch_thing(
    client: &'static Client,
    url: &'static str,
) -> impl Future<Output = Result<(), Report>> + 'static {
    async move {
        let res = client
            .get(url)
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
            .send()
            .await?
            .error_for_status()?;
        info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
        Ok(())
    }
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    const URL_1: &str = "https://api.github.com/users/aofdev";
    const URL_2: &str = "https://api.github.com/users/aofdev/repos";

    setup()?;

    let client = Client::new();
    let leaked_client = Box::leak(Box::new(client));

    let fut1 = fetch_thing(leaked_client, URL_1);
    let fut2 = fetch_thing(leaked_client, URL_2);

    tokio::spawn(fut1);
    tokio::spawn(fut2);

    tokio::time::sleep(Duration::from_secs(3)).await;

    Ok(())
}
