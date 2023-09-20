use std::error::Error;

use tcp::start_listener;

mod buffer;
mod cache;
mod tcp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    start_listener(5591).await
}
