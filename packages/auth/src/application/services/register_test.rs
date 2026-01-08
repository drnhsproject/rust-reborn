#[cfg(test)]
mod tests {
    use crate::application::dto::RegisterRequest;
    use crate::application::password_hasher::PasswordHasher;
    use crate::application::use_case::register_user::RegisterUserUseCase;
    use crate::domain::entity::User;
    use crate::domain::repository::user_repository::UserRepository;
    use crate::domain::value_objects::{Email, HashedPassword};
    use async_trait::async_trait;
    use rust_reborn_contracts::common::CodeGenerator;
    use rust_reborn_contracts::{AppError, Result};
    use std::sync::Arc;

    struct FakeUserRepository {
        email_exists: bool,
        username_exists: bool,
        saved_user: std::sync::Mutex<Option<User>>,
        should_fail: bool,
    }

    impl FakeUserRepository {
        fn new() -> Self {
            Self {
                email_exists: false,
                username_exists: false,
                saved_user: std::sync::Mutex::new(None),
                should_fail: false,
            }
        }

        fn with_email_exists() -> Self {
            Self {
                email_exists: true,
                ..Self::new()
            }
        }

        fn with_username_exists() -> Self {
            Self {
                username_exists: true,
                ..Self::new()
            }
        }

        fn with_failure() -> Self {
            Self {
                should_fail: true,
                ..Self::new()
            }
        }
    }

    #[async_trait]
    impl UserRepository for FakeUserRepository {
        async fn find_by_email(&self, _: &str) -> Result<Option<User>> {
            if self.should_fail {
                return Err(AppError::internal("database error"));
            }
            Ok(if self.email_exists {
                Some(create_test_user())
            } else {
                None
            })
        }

        async fn find_by_id(&self, id: i64) -> Result<Option<User>> {
            if self.should_fail {
                return Err(AppError::internal("database error"));
            }
            if id == 1 {
                Ok(Some(create_test_user()))
            } else {
                Ok(None)
            }
        }

        async fn find_by_username(&self, _: &str) -> Result<Option<User>> {
            if self.should_fail {
                return Err(AppError::internal("database error"));
            }
            Ok(if self.username_exists {
                Some(create_test_user())
            } else {
                None
            })
        }

        async fn update(&self, user: &User) -> Result<User> {
            if self.should_fail {
                return Err(AppError::internal("database error"));
            }
            Ok(user.clone())
        }

        async fn save(&self, user: &mut User) -> Result<()> {
            if self.should_fail {
                return Err(AppError::internal("database error"));
            }
            user.id = Some(1);
            self.saved_user.lock().unwrap().replace(user.clone());
            Ok(())
        }
    }

    struct FakeCodeGenerator;

    impl CodeGenerator for FakeCodeGenerator {
        fn generate(&self, _: &str) -> String {
            "USR-TEST-001".to_string()
        }
    }

    struct FakePasswordHasher;

    #[async_trait]
    impl PasswordHasher for FakePasswordHasher {
        fn hash(&self, _: &str) -> Result<String> {
            Ok("hashed-password".to_string())
        }

        fn verify(&self, _raw: &str, _hashed: &str) -> Result<bool> {
            Ok(true)
        }
    }

    fn create_test_user() -> User {
        User {
            id: Some(1),
            code: "USR-TEST-001".to_string(),
            email: Email::new("test@example.com".to_string()).unwrap(),
            username: "testuser".to_string(),
            password: HashedPassword::from("hashed_password".to_string()),
            full_name: Some("Test User".to_string()),
            is_active: true,
            is_verified: false,
            activation_key: None,
            reset_key: None,
            reset_key_expires_at: None,
            reset_date: None,
            status: 1,
            created_by: None,
            updated_by: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
            last_login_at: None,
        }
    }

    fn create_register_request() -> RegisterRequest {
        RegisterRequest {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            password: "Password@123".to_string(),
            full_name: Some("Test User".to_string()),
        }
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let repo = Arc::new(FakeUserRepository::new());
        let password_hasher = Arc::new(FakePasswordHasher);
        let code_generator = Arc::new(FakeCodeGenerator);

        let use_case = RegisterUserUseCase::new(repo.clone(), password_hasher, code_generator);
        let request = create_register_request();

        let result = use_case.execute(request).await;
        println!("{:?} ", result);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, 1);
        assert_eq!(response.email, "test@example.com");
        assert_eq!(response.username, "testuser");
        assert_eq!(response.full_name, Some("Test User".to_string()));
        assert!(!response.is_verified);

        let saved_user = repo.saved_user.lock().unwrap();
        assert!(saved_user.is_some());
    }

    #[tokio::test]
    async fn test_register_user_email_already_exists() {
        let repo = Arc::new(FakeUserRepository::with_email_exists());
        let password_hasher = Arc::new(FakePasswordHasher);
        let code_generator = Arc::new(FakeCodeGenerator);

        let use_case = RegisterUserUseCase::new(repo, password_hasher, code_generator);
        let request = create_register_request();

        let result = use_case.execute(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Conflict(msg) => assert_eq!(msg, "email already registered"),
            _ => panic!("expected conflict error"),
        }
    }

    #[tokio::test]
    async fn test_register_user_username_already_exists() {
        let repo = Arc::new(FakeUserRepository::with_username_exists());
        let password_hasher = Arc::new(FakePasswordHasher);
        let code_generator = Arc::new(FakeCodeGenerator);

        let use_case = RegisterUserUseCase::new(repo, password_hasher, code_generator);
        let request = create_register_request();

        let result = use_case.execute(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Conflict(msg) => assert_eq!(msg, "username already taken"),
            _ => panic!("Expected Conflict error"),
        }
    }

    #[tokio::test]
    async fn test_register_user_invalid_email() {
        let repo = Arc::new(FakeUserRepository::new());
        let password_hasher = Arc::new(FakePasswordHasher);
        let code_generator = Arc::new(FakeCodeGenerator);

        let use_case = RegisterUserUseCase::new(repo, password_hasher, code_generator);
        let mut request = create_register_request();
        request.email = "invalid-email".to_string();

        let result = use_case.execute(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert!(msg.contains("invalid email")),
            _ => panic!("expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_register_user_weak_password() {
        let repo = Arc::new(FakeUserRepository::new());
        let password_hasher = Arc::new(FakePasswordHasher);
        let code_generator = Arc::new(FakeCodeGenerator);

        let use_case = RegisterUserUseCase::new(repo, password_hasher, code_generator);
        let mut request = create_register_request();
        request.password = "weak".to_string();

        let result = use_case.execute(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(_) => {}
            _ => panic!("expected BadRequest error"),
        }
    }

    #[tokio::test]
    async fn test_register_user_repository_error() {
        let repo = Arc::new(FakeUserRepository::with_failure());
        let password_hasher = Arc::new(FakePasswordHasher);
        let code_generator = Arc::new(FakeCodeGenerator);

        let use_case = RegisterUserUseCase::new(repo, password_hasher, code_generator);
        let request = create_register_request();

        let result = use_case.execute(request).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Internal(msg) => assert_eq!(msg, "database error"),
            _ => panic!("expected internal server error"),
        }
    }

    #[tokio::test]
    async fn test_repository_find_by_email_found() {
        let repo = Arc::new(FakeUserRepository::with_email_exists());

        let result = repo.find_by_email("test@example.com").await;

        assert!(result.is_ok());
        let found_user = result.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().email.value(), "test@example.com");
    }

    #[tokio::test]
    async fn test_repository_find_by_email_not_found() {
        let repo = Arc::new(FakeUserRepository::new());

        let result = repo.find_by_email("notfound@example.com").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_repository_find_by_username_found() {
        let repo = Arc::new(FakeUserRepository::with_username_exists());

        let result = repo.find_by_username("testuser").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_repository_find_by_username_not_found() {
        let repo = Arc::new(FakeUserRepository::new());

        let result = repo.find_by_username("notfound").await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_repository_find_by_id_found() {
        let repo = Arc::new(FakeUserRepository::new());

        let result = repo.find_by_id(1).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_repository_find_by_id_not_found() {
        let repo = Arc::new(FakeUserRepository::new());

        let result = repo.find_by_id(999).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_repository_update_success() {
        let repo = Arc::new(FakeUserRepository::new());
        let mut user = create_test_user();
        user.full_name = Some("Updated Name".to_string());

        let result = repo.update(&user).await;

        assert!(result.is_ok());
        let saved_user = result.unwrap();
        assert_eq!(saved_user.full_name, Some("Updated Name".to_string()));
    }

    #[tokio::test]
    async fn test_repository_save_success() {
        let repo = Arc::new(FakeUserRepository::new());
        let mut user = create_test_user();
        user.id = None;

        let result = repo.save(&mut user).await;

        assert!(result.is_ok());
        assert_eq!(user.id, Some(1));
    }

    #[tokio::test]
    async fn test_password_hasher_hash() {
        let hasher = FakePasswordHasher;

        let result = hasher.hash("password123");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hashed-password");
    }

    #[tokio::test]
    async fn test_password_hasher_verify() {
        let hasher = FakePasswordHasher;

        let result = hasher.verify("password123", "hashed-password");

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_code_generator_generate() {
        let generator = FakeCodeGenerator;

        let result = generator.generate("usr");

        assert_eq!(result, "USR-TEST-001");
    }
}
