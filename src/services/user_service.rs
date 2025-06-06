use crate::database::sqlite::SqliteDatabase;
use crate::errors::{AppError, Result};
use crate::models::user::{CreateUserRequest, User, UserResponse};
use crate::utils::crypto::PasswordManager;
use crate::utils::validation::Validator;
use chrono::Utc;
use uuid::Uuid;
use std::env;

pub struct UserService {
    db: SqliteDatabase,
}

impl UserService {
    pub async fn new() -> Result<Self> {
        // Get current directory and create database path
        let current_dir = env::current_dir()
            .map_err(|e| AppError::DatabaseError(format!("Failed to get current directory: {}", e)))?;
        
        let db_path = current_dir.join("stellar_wallet.db");
        let db_path_str = db_path.to_string_lossy();
        
        println!("📂 Database path: {}", db_path_str);
        
        let db = SqliteDatabase::new(&db_path_str).await?;
        Ok(Self { db })
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse> {
        // Validate input
        Validator::validate_email(&request.email)?;
        Validator::validate_username(&request.username)?;
        Validator::validate_password(&request.password)?;

        // Hash password
        let password_hash = PasswordManager::hash_password(&request.password)?;

        // Create user
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        
        let user = User {
            id: user_id,
            email: request.email.clone(),
            username: request.username.clone(),
            password_hash,
            is_verified: false,
            stellar_public_key: None,
            created_at: now,
            updated_at: now,
        };

        // Save to database
        self.db.create_user(&user).await?;

        Ok(user.into())
    }

    pub async fn authenticate_user(&self, email_or_username: &str, password: &str) -> Result<UserResponse> {
        // Try to find user by email first, then by username
        let user = if let Some(user) = self.db.get_user_by_email(email_or_username).await? {
            user
        } else if let Some(user) = self.db.get_user_by_username(email_or_username).await? {
            user
        } else {
            return Err(AppError::AuthenticationError("Invalid email/username or password".to_string()));
        };

        // Verify password
        if !PasswordManager::verify_password(password, &user.password_hash)? {
            return Err(AppError::AuthenticationError("Invalid email/username or password".to_string()));
        }

        println!("✅ Authentication successful for user: {}", user.username);
        Ok(user.into())
    }

    pub async fn get_user_count(&self) -> Result<i64> {
        self.db.get_user_count().await
    }
}
