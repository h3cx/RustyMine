use crate::prelude::*;
use std::sync::Arc;

use crate::{
    core,
    domain::user::{InternalNewUser, NewUser, User},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, StatusCode> {
    let user = core::user_routines::create(state, new_user).await?;
    Ok(Json(user))
}

pub async fn get_all(State(state): State<Arc<AppState>>) -> Result<Json<Vec<User>>, StatusCode> {
    let users = core::user_routines::get_all(state).await?;
    Ok(Json(users))
}
