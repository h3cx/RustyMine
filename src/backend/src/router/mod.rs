pub mod middleware;
use anyhow::Result;
use axum::{Json, Router, http::StatusCode, middleware::from_fn, routing::get};
use serde_json::{Value, json};

pub async fn init_router() -> Result<Router> {
    let router = Router::new().route(
        "/api/ping",
        get(ping).layer(from_fn(middleware::auth_middleware)),
    );
    Ok(router)
}

async fn ping() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "response": "pong"})))
}
