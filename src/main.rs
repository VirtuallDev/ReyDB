use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub const CRLF: &str = "\r\n";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:5591").await?;

    println!("Started ReyDB server...");
    println!("Connect using the CLI tool or API handlers!");

    while let Ok((mut stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            client_handler(&mut stream).await;
        });
    }

    Ok(())
}

async fn client_handler(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        let read_bytes = stream.read(&mut buffer).await?;
        if read_bytes == 0 {
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..read_bytes])
            .trim()
            .to_owned();
        let (command, args) = parse_command(&message).await?;

        let response = match command.as_str() {
            "ECHO" => args,
            _ => panic!(""),
        } + CRLF;

        stream.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

async fn parse_command(message: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut splited = message.splitn(2, ' ');
    let command = splited.next().unwrap_or_default();
    let args = splited.next().unwrap_or_default();

    Ok((command.to_owned(), args.to_owned()))
}
