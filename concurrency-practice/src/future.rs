use color_eyre::Report;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use std::future::Future;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub const URL_1: &str = "https://api.github.com/users/aofdev";

fn fetch_thing<'a>(
    client: &'a Client,
    url: &'a str,
) -> impl Future<Output = Result<(), Report>> + 'a {
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

fn type_name_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

#[tokio::main]
async fn main() -> Result<(), Report> {
    setup()?;

    info!("Building that fetch future...");
    let client = Client::new();
    let fut = fetch_thing(&client, URL_1);
    info!(
        type_name = type_name_of(&fut),
        "That fetch future has a type.."
    );
    info!("Awaiting that fetch future...");
    fut.await?;
    info!("Done awaiting that fetch future");

    Ok(())
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
