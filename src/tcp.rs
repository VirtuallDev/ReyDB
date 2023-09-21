use std::error::Error;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use crate::buffer::TCPBuffer;
use crate::cache::{self, Cache};

pub const CRLF: &str = "\r\n";

#[derive(Clone)]
pub(crate) struct TcpManager {
    cache: Cache,
    port: u16,
    host: &'static str,
    _tcp_listener: Arc<Mutex<TcpListener>>,
}

impl TcpManager {
    /// Constructor
    pub async fn new(
        cache: Cache,
        port: u16,
        host: &'static str,
    ) -> Result<TcpManager, Box<dyn Error>> {
        let temp_tcp_lis = TcpListener::bind((host, port)).await?;
        let listener = Arc::new(Mutex::new(temp_tcp_lis));
        Ok(TcpManager {
            cache: cache,
            port: port,
            host: host,
            _tcp_listener: listener,
        })
    }

    /// Start server running
    pub async fn run_server(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Started ReyDB Server...");
        println!("Connect using the CLI or API wrappers!");

        let borrowed_tcp = &self._tcp_listener.lock().unwrap();
        let mut borrowed_self = self.clone();
        while let Ok((mut stream, _)) = borrowed_tcp.accept().await {
            let res = borrowed_self.client_handler(&mut stream).await;
            match res {
                Ok(_) => println!("Received user connection succesfully"),
                Err(err) => panic!("{}", err),
            }
        }

        Ok(())
    }

    pub async fn client_handler(&mut self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

/// A simple client handler that handles the conversation between the server and the client (WILL BE IMPROVED)
async fn client_handler(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = TCPBuffer::new();

    loop {
        let _ = buffer.read_to_buffer(stream).await; // reads to the buffer struct
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
