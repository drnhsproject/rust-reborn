use crate::domain::repositories::user_repository::UserRepository;
use std::sync::Arc;

pub struct AuthService<R: UserRepository> {
    user_repository: Arc<R>
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(
        user_repository: Arc<R>,
    ) -> Self {
        Self {
            user_repository,
        }
    }
}
