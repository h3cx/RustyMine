use axum::{extract::Request, middleware::Next, response::IntoResponse};
use tracing::{debug, error, info, warn};

pub async fn auth_middleware(request: Request, next: Next) -> impl IntoResponse {
    debug!("auth_middleware entry");
    let response = next.run(request).await;
    debug!("auth_middleware exit");
    response
}
