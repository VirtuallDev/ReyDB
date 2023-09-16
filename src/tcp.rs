use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use crate::buffer::TCPBuffer;

pub const CRLF: &str = "\r\n";

pub async fn start_listener(port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    println!("Started ReyDB server...");
    println!("Connect using the CLI tool or API handlers!");

    while let Ok((mut stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let res = client_handler(&mut stream).await;
            match res {
                Ok(_) => println!(),
                Err(e) => panic!("{}", e),
            }
        });
    }

    Ok(())
}

async fn client_handler(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = TCPBuffer::new();

    loop {
        buffer.read_to_buffer(stream).await; // reads to the buffer struct
        println!("{:?}", buffer.get_mut_buffer());
        let message_res = String::from_utf8_lossy(&buffer.get_mut_buffer());

        let (command, args) = parse_command(&message_res).await?;

        let response = match command {
            "ECHO" => args[2],
            _ => panic!("Debug: Command is invalid!"),
        }
        .to_owned()
            + CRLF;

        stream.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

async fn parse_command(message: &str) -> Result<(&str, Vec<&str>), Box<dyn Error>> {
    let splited = message.split(CRLF).peekable().collect::<Vec<&str>>(); // \r\nCOMMAND\r\nDATA\r\n
    Ok((splited[1], splited.to_owned()))
}
