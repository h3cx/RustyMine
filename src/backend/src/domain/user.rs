use std::fmt::Display;

use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::auth;

#[derive(Debug, Clone, Deserialize)]
pub struct NewUser {
    username: String,
    email: Option<String>,
    password: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InternalUser {
    username: String,
    email: Option<String>,
    password_hash: String,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct User {
    username: String,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Debug)]
pub enum UserConversionError {
    HashFailed(password_hash::Error),
}

impl TryFrom<NewUser> for InternalUser {
    type Error = UserConversionError;
    fn try_from(value: NewUser) -> Result<Self, Self::Error> {
        let password_hash =
            auth::hash_password(&value.password).map_err(|e| UserConversionError::HashFailed(e))?;
        Ok(Self {
            username: value.username,
            email: value.email,
            password_hash: password_hash,
            first_name: value.first_name,
            last_name: value.last_name,
        })
    }
}

impl From<InternalUser> for User {
    fn from(value: InternalUser) -> Self {
        Self {
            username: value.username,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
        }
    }
}

impl Display for UserConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserConversionError::HashFailed(e) => write!(f, "failed to hash password: {e}"),
        }
    }
}
