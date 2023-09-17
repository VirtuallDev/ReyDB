use std::error::Error;
use std::str::FromStr;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use crate::buffer::TCPBuffer;

pub const CRLF: &str = "\r\n";

/// A listener function (WILL BE IMPROVED)
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

/// A simple client handler that handles the conversation between the server and the client (WILL BE IMPROVED)
async fn client_handler(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = TCPBuffer::new();

    loop {
        buffer.read_to_buffer(stream).await; // reads to the buffer struct
        let message_res = String::from_utf8_lossy(buffer.get_mut_buffer());

        let (command, args) = parse_command(&message_res).await?;
        let str_args = &args.join(" ");

        let response = CRLF.to_string()
            + match command {
                "ECHO" => str_args,
                _ => "INVALID",
            }
            + CRLF;

        stream.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

/// A function that parses the command, returns the command name and the given arguments (WILL BE IMPROVED FOR DATA LOADING)
async fn parse_command(message: &str) -> Result<(&str, Vec<&str>), Box<dyn Error>> {
    let mut splited = message.split(CRLF).peekable().collect::<Vec<&str>>(); // \r\nCOMMAND\r\nDATA\r\n
    splited.retain(|s| !s.trim().is_empty());
    let mut args = splited.clone();
    args.remove(0);
    Ok((splited[0], args.to_owned()))
}
