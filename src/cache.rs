use std::error::Error;

type CacheItem = Vec<u8>;

pub struct Cache {
    _cached_resps: Vec<CacheItem>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            _cached_resps: vec![],
        }
    }

    pub fn save_response(&mut self, data: Vec<u8>) {
        self._cached_resps.push(data);
    }
}
