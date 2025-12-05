use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum UserActions {
    ManageUsers,
    Login,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct UserPermissions {
    pub root: bool,
    pub permissions: HashSet<UserActions>,
}

impl Default for UserPermissions {
    fn default() -> Self {
        Self {
            root: false,
            permissions: Vec::new(),
        }
    }
}

impl UserPermissions {
    pub fn root() -> Self {
        Self {
            root: true,
            permissions: Vec::new(),
        }
    }
}
