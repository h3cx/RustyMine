use std::sync::Arc;

use crate::{domain::user::NewUser, state::AppState};

pub async fn create(state: Arc<AppState>, new_user: NewUser) -> Result<User> {}
