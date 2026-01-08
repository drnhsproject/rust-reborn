use crate::application::port::auth_context::AuthContext;

#[derive(Clone)]
pub struct RequestAuthContext {
    user_id: Option<i64>,
}

impl RequestAuthContext {
    pub fn authenticated(user_id: i64) -> Self {
        Self {
            user_id: Some(user_id),
        }
    }

    pub fn anonymous() -> Self {
        Self {
            user_id: None,
        }
    }
}

impl AuthContext for RequestAuthContext {
    fn user_id(&self) -> Option<i64> {
        self.user_id
    }
}