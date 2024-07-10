use tracing::Level;
use tracing_subscriber::FmtSubscriber;
mod database;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // ----------------------------------------------------------------

    database::run_all_tests().await;
}