use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug)]
pub enum UserActions {
    Root,
    ManageUsers,
    Login,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct UserPermissions {
    pub root: bool,
    pub manage_users: bool,
    pub login: bool,
}

impl Default for UserPermissions {
    fn default() -> Self {
        Self {
            root: false,
            manage_users: false,
            login: false,
        }
    }
}
