use std::error::Error;

use cache::Cache;
use tcp::start_listener;

mod buffer;
mod cache;
mod tcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cache = Cache::new();
    let tcp_manager = tcp::TcpManager::new(cache, 5591, "0.0.0.0");
    Ok(())
}
