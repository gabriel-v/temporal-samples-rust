use dotenvy::dotenv;

use timer::worker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    worker::start_worker().await?;
    Ok(())
}
