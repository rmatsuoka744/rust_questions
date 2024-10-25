use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::{Value, json};
use std::env;

async fn handle_client(mut socket: TcpStream, server_header: String) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer).await {
        Ok(n) if n == 0 => return,
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Received request: {}", request);

            let json_data = if let Some(data) = parse_json_from_request(&request) {
                println!("Received JSON: {:?}", data);
                data
            } else {
                eprintln!("Failed to parse JSON from request");
                return;
            };

            let response = json!({
                "header": server_header,
                "body": format!("{} - Server added content", json_data["body"].as_str().unwrap_or(""))
            });

            let response_str = response.to_string();

            if let Err(e) = socket.write_all(response_str.as_bytes()).await {
                eprintln!("Failed to write to socket; err = {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from socket; err = {:?}", e);
        }
    }
}


fn parse_json_from_request(request: &str) -> Option<Value> {
    if let Some(start) = request.find("{") {
        if let Some(end) = request.rfind("}") {
            let json_str = &request[start..=end];
            return serde_json::from_str(json_str).ok();
        }
    }
    None
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: server <header>");
        return Ok(());
    }
    let server_header = args[1].clone();

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080");

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("New connection from {}", addr);
                let server_header = server_header.clone();
                tokio::spawn(async move {
                    handle_client(socket, server_header).await;
                });
            }
            Err(e) => eprintln!("Failed to accept connection; err = {:?}", e),
        }
    }
}