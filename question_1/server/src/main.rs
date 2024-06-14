use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;


async fn handle_client(mut socket: TcpStream) {
    let mut buffer = [0; 1024];

    match socket.read(&mut buffer).await {
        Ok(n) if n == 0 => return,
        Ok(n) => {
            let request = String::from_utf8_lossy(&buffer[..n]);
            println!("Received request: {}", request);

            if let Some(json_data) = parse_json_from_request(&request) {
                println!("Received JSON: {:?}", json_data);
            }

            let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nTank you JSON!";

            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("Failed to write to socket; err = {:?}", e);
                return;
            }

            if let Err(e) = socket.shutdown().await {
                eprintln!("Failed to shutdown socket; err = {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from socket; err = {:?}", e);
            return;
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
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on port 8080");

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("New connection from {}", addr);
                tokio::spawn(async move {
                    handle_client(socket).await;
                });
            }
            Err(e) => eprintln!("Failed to accept connection; err = {:?}", e),
        }
    }
}