use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(max = 100))]
    pub full_name: Option<String>,
}
