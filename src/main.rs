use dotenv::dotenv;
mod modules;

use modules::consumer::consumer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    consumer().await;

    std::future::pending::<()>().await;
}
