use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub const CRLF: &str = "\r\n";
const BUFF_SIZE: usize = 1024; // temporary
pub type TCPBuffer = [u8; BUFF_SIZE]; // TODO: change the buffer to be dynamic

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
    let mut buffer: TCPBuffer = [0; BUFF_SIZE];

    loop {
        let read_bytes = stream.read(&mut buffer).await?;
        if read_bytes == 0 {
            break;
        }

        let message = String::from_utf8_lossy(&buffer[..read_bytes]).to_owned();
        let (command, args) = parse_command(&message).await?;

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
    println!("{:?}", &splited);
    Ok((splited[1], splited.to_owned()))
    // Ok((command.to_owned(), args.to_owned()))
}
