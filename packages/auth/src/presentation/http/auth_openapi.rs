use crate::application::dto::{AuthResponse, LoginRequest};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::presentation::http::handlers::login
    ),
    components(
        schemas(LoginRequest, AuthResponse),
    ),
    tags(
        (name = "Authentication", description = "Authentication management APIs")
    )
)]
pub struct AuthApiDoc;
