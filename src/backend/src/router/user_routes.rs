use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;
use tracing::{debug, error, info, warn};

use crate::{
    domain::user::{InternalUser, NewUser, User},
    state::AppState,
};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let internal = InternalUser::try_from(new_user).map_err(|e| {
        error!("Conversion to InternalUser failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = User::from(internal);

    Ok(Json(user))
}
