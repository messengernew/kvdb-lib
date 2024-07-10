use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
mod database;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("czh started!");

    // ----------------------------------------------------------------

    database::run_all_tests().await;

    info!("czh stopped!");
}