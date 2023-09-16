use std::error::Error;

use tokio::{io::AsyncReadExt, net::TcpStream};

#[derive(Clone)]
pub struct TCPBuffer {
    _buff: Vec<u8>,
    _msg_size: usize,
}

pub const BUFFER_CAPACITY_DEFAULT: usize = 1024;
pub const SIZEOF_BIG_INT: usize = 8;

impl TCPBuffer {
    /* Constructor */
    pub fn new() -> TCPBuffer {
        TCPBuffer {
            _buff: vec![],
            _msg_size: BUFFER_CAPACITY_DEFAULT,
        }
    }

    /* Saves the streamed data dynamically into the array  */
    pub async fn read_to_buffer(&mut self, stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
        self.reset();

        let mut buffer = [0; SIZEOF_BIG_INT];
        let size: usize = match stream.read_exact(&mut buffer).await {
            Ok(_) => usize::from_be_bytes(buffer),
            Err(_) => 0 as usize,
        }
        .to_owned();

        self._msg_size += size;
        self._buff = vec![0; size];

        match stream.read_exact(&mut self._buff[..size]).await {
            Ok(_) => Ok(()),
            Err(_) => {
                panic!("failed reading")
            }
        }
    }

    /* Get the buffer */
    pub fn get_mut_buffer(&mut self) -> &Vec<u8> {
        &self._buff
    }

    /* Get the buffer as readonly */
    pub fn get_buffer(self) -> Vec<u8> {
        self._buff.clone()
    }

    /* Resets the buffer's content and size, including capacity.  */
    pub fn reset(&mut self) {
        self._buff = vec![];

        self._msg_size = BUFFER_CAPACITY_DEFAULT;
    }
}
