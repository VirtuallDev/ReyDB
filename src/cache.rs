use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct CacheItem {
    pub value: Vec<u8>,
    pub expires_at: SystemTime,
}

/// Represents 1 day in milliseconds
pub const EXPIRATION_TIME: u64 = ((24 * 60) * 60) * 1000;

impl CacheItem {
    pub fn from(buffer: Vec<u8>) -> CacheItem {
        CacheItem {
            value: buffer.clone(),
            expires_at: SystemTime::now() + Duration::from_millis(EXPIRATION_TIME),
        }
    }

    /// This function checks if the cache item passed the expiration date
    pub fn is_expired(&mut self) -> bool {
        match self.expires_at.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(durr) => durr < SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            Err(_) => false,
        }
    }
}
#[derive(Clone)]
pub struct Cache {}

impl Cache {
    pub fn new() -> Cache {
        Cache {}
    }
}
