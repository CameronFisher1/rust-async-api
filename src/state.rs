use std::sync::Arc;

use crate::service::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}

impl AppState {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }
}
