use std::env;
use std::io;
use thiserror::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use reqwest::Error as ReqwestError;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("I/O Error")]
    Io(#[from] io::Error),
    #[error("Request Error")]
    Reqwest(#[from] ReqwestError),
}

async fn download_html(url: &str) -> Result<String, DownloadError> {
    let response = reqwest::get(url).await?;
    let html = response.text().await?;
    Ok(html)
}

async fn save_html(prefix: &str, number: usize, html: &str) -> Result<(), DownloadError> {
    let file_name = format!("{}_{}.html", prefix, number);
    let mut file = File::create(file_name).await?;
    file.write_all(html.as_bytes()).await?;
    Ok(())
}

async fn process_url(url: &str, prefix: &str, number: usize) -> Result<(), DownloadError> {
    let html = download_html(url).await?;
    save_html(prefix, number, &html).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    // cargo run -- example https://en.wikipedia.org/wiki/Example.com https://ja.wikipedia.org/wiki/Example.com
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: <prefix> <url1> <url2> ...");
        return Ok(());
    }
    
    let prefix = &args[1];
    let urls = &args[2..];

    let tasks: Vec<_> = urls.iter().enumerate().map(|(i, url)| {
        let prefix = prefix.clone();
        let url = url.clone();
        tokio::spawn(async move {
            match process_url(&url, &prefix, i).await {
                Ok(_) => println!("Successfully downloaded and saved: {}", url),
                Err(e) => eprintln!("Failed to process {}: {:?}", url, e),
            }
        })
    }).collect();

    futures::future::join_all(tasks).await;
    
    Ok(())
}
