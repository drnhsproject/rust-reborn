use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthClaims {
    pub user_id: String,
    pub roles: Vec<String>,
}
