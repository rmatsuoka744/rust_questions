use reqwest::Error as ReqwestError;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use thiserror::Error;
use tokio::task;

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
    let mut file = File::create(file_name)?;
    file.write_all(html.as_bytes())?;
    Ok(())
}

async fn process_url(url: &str, prefix: &str, number: usize) -> Result<(), DownloadError> {
    let html = download_html(url).await?;
    save_html(prefix, number, &html).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    let url = "https://en.wikipedia.org/wiki/Example.com";
    let prefix = String::from("example");
    let number = 1;

    match process_url(&url, &prefix, number).await {
        Ok(_) => println!("Successfully downloaded and saved: {}", url),
        Err(e) => eprintln!("Failed to process {}: {:?}", url, e),
    }
    Ok(())
}
