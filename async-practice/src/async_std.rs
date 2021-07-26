use async_std::{fs::File, io, prelude::*, task};
use futures::stream::FuturesUnordered;
use std::thread;
use std::time::Instant;

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

async fn get_web(url: &str) -> Result<String, surf::Error> {
    let mut response = surf::get(url).await?;
    Ok(response.body_string().await?)
}

async fn download_url(url: &str) -> Result<String, surf::Error> {
    println!("Downloading {} on thread {:?}", url, thread::current().id());

    let mut result = surf::get(url).await?;
    let body = result.body_string().await?;

    println!(
        "    Downloaded {}, returning body of length {} ",
        url,
        body.len()
    );
    Ok(body)
}

fn downloading_urls_on_one_thread(urls: &[&str]) {
    let mut futures = urls
        .iter()
        .map(|url| download_url(url))
        .collect::<FuturesUnordered<_>>();

    task::block_on(async {
        while let Some(return_val) = futures.next().await {
            match return_val {
                Ok(_) => {}
                Err(e) => println!("    Got error {:?}", e),
            }
        }
    });
}

fn downloading_urls_on_multiple_threads(urls: &[&str]) {
    let mut tasks = Vec::with_capacity(urls.len());

    for url in urls.iter() {
        let url = url.to_string();
        tasks.push(task::spawn(async move {
            match download_url(&url).await {
                Ok(_) => {}
                Err(e) => println!("    Got error {:?}", e),
            }
        }))
    }

    task::block_on(async {
        for t in tasks {
            t.await;
        }
    });
}

fn main() {
    // Spawn this runs a background task and then waits for its completion, blocking the main thread.
    let reader_task = task::spawn(async {
        let result = read_file("Cargo.toml").await;
        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error reading file: {:?}", e),
        }
    });
    println!("Started reader_task!");
    task::block_on(reader_task);
    println!("Stopped reader_task!");

    let get_web_task = task::spawn(async {
        let result = get_web("https://www.rust-lang.org").await;
        match result {
            Ok(s) => println!("{}", s),
            Err(e) => println!("Error get: {:?}", e),
        }
    });

    println!("Started get_web_task!");
    task::block_on(get_web_task);
    println!("Stopped get_web_task!");

    // Download urls on one thread and multiple threads
    const URLS: [&str; 10] = [
        "https://jsonplaceholder.typicode.com/todos/1",
        "https://jsonplaceholder.typicode.com/todos/2",
        "https://jsonplaceholder.typicode.com/todos/3",
        "https://jsonplaceholder.typicode.com/todos/4",
        "https://jsonplaceholder.typicode.com/todos/5",
        "https://jsonplaceholder.typicode.com/todos/6",
        "https://jsonplaceholder.typicode.com/todos/7",
        "https://jsonplaceholder.typicode.com/todos/8",
        "https://jsonplaceholder.typicode.com/todos/9",
        "https://jsonplaceholder.typicode.com/todos/10",
    ];

    let start_time_downloading_urls_on_one_thread = Instant::now();
    downloading_urls_on_one_thread(&URLS);
    println!(
        "Function start_time_downloading_urls_on_one_thread finished in {} ms",
        start_time_downloading_urls_on_one_thread
            .elapsed()
            .as_millis()
    );

    let start_time_downloading_urls_on_multiple_threads = Instant::now();
    downloading_urls_on_multiple_threads(&URLS);
    println!(
        "Function start_time_downloading_urls_on_multiple_threads finished in {} ms",
        start_time_downloading_urls_on_multiple_threads
            .elapsed()
            .as_millis()
    );
}
