use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt, BufReader, AsyncBufReadExt};
use serde_json::json;
use std::env;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: client <header>");
        return Ok(());
    }
    let client_header = args[1].clone();

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    println!("Enter the body content:");
    let mut body = String::new();
    let mut reader = BufReader::new(tokio::io::stdin());
    reader.read_line(&mut body).await?;

    let json_data = json!({
        "header": client_header,
        "body": body.trim()
    });

    let json_str = json_data.to_string();
    stream.write_all(json_str.as_bytes()).await?;

    tokio::spawn(async {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
        println!("\nDisconnecting from server...");
    });

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    println!("Received response from server: {}", response);

    Ok(())
}
