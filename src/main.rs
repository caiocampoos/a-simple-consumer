use dotenv::dotenv;
use tracing::info;
use tracing_subscriber;
mod modules;

use modules::consumer::consumer::consumer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    consumer().await;
    info!("Consumer started");

    std::future::pending::<()>().await;
}
