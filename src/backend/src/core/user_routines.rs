use crate::{infra::db, prelude::*};
use std::sync::Arc;

use anyhow::Result;
use axum::{Json, http::StatusCode};
use validator::Validate;

use crate::{
    domain::user::{InternalNewUser, NewUser, User},
    state::AppState,
};

pub async fn create(state: Arc<AppState>, new_user: NewUser) -> Result<User, StatusCode> {
    new_user.validate().map_err(|e| {
        error!("User validation failed: {e}");
        StatusCode::BAD_REQUEST
    })?;

    let internal = InternalNewUser::try_from(new_user).map_err(|e| {
        error!("Conversion to InternalUser failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let created_user = db::user::create(&state.db_pool, internal)
        .await
        .map_err(|e| {
            error!("Failed to create new user: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(User::from(created_user))
}

pub async fn get_all(state: Arc<AppState>) -> Result<Vec<User>, StatusCode> {
    let users = db::user::get_safe_all(&state.db_pool).await.map_err(|e| {
        error!("Failed to fetch all users: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(users)
}
