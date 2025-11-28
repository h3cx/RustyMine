use std::sync::Arc;

use anyhow::{Ok, Result};
use rustymine_daemon::{router, state::AppState};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let state = Arc::new(AppState::new());

    let app_result = router::init_router(state.clone()).await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app_result).await?;
    Ok(())
}
