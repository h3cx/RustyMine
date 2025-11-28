use axum::{
    Router,
    extract::Request,
    http::{Method, header::AUTHORIZATION},
    middleware::Next,
    response::IntoResponse,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, error, info, warn};

pub async fn auth(request: Request, next: Next) -> impl IntoResponse {
    debug!("auth_middleware entry");
    let response = next.run(request).await;
    debug!("auth_middleware exit");
    response
}

pub fn cors() -> CorsLayer {
    debug!("Generating CorsLayer");
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION])
}
