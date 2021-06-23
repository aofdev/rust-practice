use futures::future::join_all;
use log::*;
use std::{thread, time};
use tokio::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    env_logger::init();

    match app_one().await {
        Ok(_) => info!("App one done!"),
        Err(e) => error!("An app one error occurred.: {}", e),
    };

    match app_two().await {
        Ok(_) => info!("App two done!"),
        Err(e) => error!("An app two error occurred: {}", e),
    };

    match app_three().await {
        Ok(_) => info!("App three done!"),
        Err(e) => error!("An app three error occurred: {}", e),
    };
}

fn slowly(delay_ms: u64) -> reqwest::Url {
    thread::sleep(time::Duration::from_millis(delay_ms));
    let url = "https://api.github.com/users/aofdev";
    reqwest::Url::parse(&url).unwrap()
}

async fn app_one() -> Result<()> {
    info!("Starting app one!");
    let _resp1 = reqwest::get(slowly(1000)).await?;
    info!("Got response 1");
    let _resp2 = reqwest::get(slowly(2000)).await?;
    info!("Got response 2");
    Ok(())
}

async fn request(n: usize) -> Result<()> {
    reqwest::get(slowly(1000)).await?;
    info!("Got response {}", n);
    Ok(())
}

async fn app_two() -> Result<()> {
    info!("Starting app two!");
    let resp1 = task::spawn(request(1));
    let resp2 = task::spawn(request(2));

    let _ = resp1.await??;
    let _ = resp2.await??;

    Ok(())
}

// Now we want to both fetch some data and do some CPU intensive analysis on it
async fn get_and_analyze(n: usize) -> Result<(u64, u64)> {
    let response: reqwest::Response = reqwest::get(slowly(1000)).await?;
    info!("Dataset {}", n);

    let txt = response.text().await?;

    // We send our analysis work to a thread where there is no runtime running
    // so we don't block the runtime by analyzing the data
    let res = task::spawn_blocking(move || analyze(&txt)).await?;
    info!("Processed {}", n);
    Ok(res)
}

// Counting the number of ones and zeros in the bytes we get.
fn analyze(txt: &str) -> (u64, u64) {
    let txt = txt.as_bytes();
    // Let's spend as much time as we can and count them in two passes
    let ones = txt
        .iter()
        .fold(0u64, |acc, b: &u8| acc + b.count_ones() as u64);
    let zeros = txt
        .iter()
        .fold(0u64, |acc, b: &u8| acc + b.count_zeros() as u64);
    (ones, zeros)
}

async fn app_three() -> Result<()> {
    info!("Starting app three!");
    let round = 10;
    let mut futures = vec![];

    for i in 1..=round {
        let fut = task::spawn(get_and_analyze(i));
        futures.push(fut);
    }

    let results = join_all(futures).await;

    let mut total_ones = 0;
    let mut total_zeros = 0;

    for result in results {
        // `spawn_blocking` returns a `JoinResult` we need to unwrap first
        let ones_res: Result<(u64, u64)> = result?;
        let (ones, zeros) = ones_res?;
        total_ones += ones;
        total_zeros += zeros;
    }

    info!(
        "Ratio of ones/zeros: {:.02}",
        total_ones as f64 / total_zeros as f64
    );
    Ok(())
}
