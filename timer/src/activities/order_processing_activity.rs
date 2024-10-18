use log::info;
use rand::Rng;
use temporal_sdk::{ActContext, ActivityError};

pub async fn order_processing_activity(
    _ctx: ActContext,
    _payload: Option<String>,
) -> Result<(), ActivityError> {
    info!("Order processing activity started");

    let time_needed = rand::thread_rng().gen_range(1..10);
    // let time_needed = 5; // hard code this for testing
    info!("Processing will take {} seconds", time_needed);
    tokio::time::sleep(tokio::time::Duration::from_secs(time_needed)).await;

    info!("Order processing activity completed");

    Ok(())
}
