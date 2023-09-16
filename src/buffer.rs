use std::error::Error;

pub struct TCPBuffer {
    _buff: Vec<u8>,
    _msg_size: usize,
    _msg_capacity: usize,
}

pub const BUFFER_CAPACITY_DEFAULT: usize = 1024;

impl TCPBuffer {
    pub fn new() -> TCPBuffer {
        TCPBuffer {
            _buff: Vec::with_capacity(BUFFER_CAPACITY_DEFAULT),
            _msg_size: BUFFER_CAPACITY_DEFAULT,
            _msg_capacity: BUFFER_CAPACITY_DEFAULT,
        }
    }

    /* TODO: fill the buffer with the given data  */
    pub fn fill(&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /* Resets the buffer's content and size, including capacity.  */
    pub fn reset(&mut self) -> Result<(), Box<dyn Error>> {
        self._buff = Vec::with_capacity(BUFFER_CAPACITY_DEFAULT);

        self._msg_size = 0;
        self._msg_capacity = BUFFER_CAPACITY_DEFAULT;

        Ok(())
    }
}
