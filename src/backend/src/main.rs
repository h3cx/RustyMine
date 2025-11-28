use anyhow::{Ok, Result};
use rustymine_daemon::router;
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let app_result = router::init_router().await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app_result).await.unwrap();
    Ok(())
}
