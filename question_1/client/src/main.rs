use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    let json_data = r#"
    {
        "header": "header_context",
        "body": "body_context"
    }
    "#;

    let request = format!(
        "POST / HTTP/1.1\r\nContent-Length: {}\r\n\r\n{}",
        json_data.len(),
        json_data
    );

    stream.write_all(request.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    println!("Received response: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}