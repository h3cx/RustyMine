use std::collections::HashMap;

use crate::domain::user::InternalNewUser;

pub struct AppState {
    pub users: HashMap<String, InternalNewUser>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}
