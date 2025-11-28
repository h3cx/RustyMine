use std::collections::HashMap;

use crate::domain::user::InternalUser;

pub struct AppState {
    pub users: HashMap<String, InternalUser>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}
