use std::error::Error;

use cache::Cache;
use rusqlite::Connection;
use tcp::TcpManager;

mod buffer;
mod cache;
mod database;
mod tcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cache = Cache::new();
    let mut tcp_manager = tcp::TcpManager::new(cache, 5591, "0.0.0.0").await?;
    tcp_manager.run_server().await
}
