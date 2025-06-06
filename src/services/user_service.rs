use crate::errors::{AppError, Result};
use crate::models::user::{CreateUserRequest, User, UserResponse};
use crate::utils::crypto::PasswordManager;
use crate::utils::validation::Validator;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct UserService {
    users: Arc<Mutex<HashMap<Uuid, User>>>,
    email_index: Arc<Mutex<HashMap<String, Uuid>>>,
    username_index: Arc<Mutex<HashMap<String, Uuid>>>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            email_index: Arc::new(Mutex::new(HashMap::new())),
            username_index: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse> {
        // Validate input
        Validator::validate_email(&request.email)?;
        Validator::validate_username(&request.username)?;
        Validator::validate_password(&request.password)?;

        // Check if email already exists
        {
            let email_index = self.email_index.lock()
                .map_err(|_| AppError::InternalError("Failed to acquire email index lock".to_string()))?;
            
            if email_index.contains_key(&request.email) {
                return Err(AppError::ValidationError("Email already exists".to_string()));
            }
        }

        // Check if username already exists
        {
            let username_index = self.username_index.lock()
                .map_err(|_| AppError::InternalError("Failed to acquire username index lock".to_string()))?;
            
            if username_index.contains_key(&request.username) {
                return Err(AppError::ValidationError("Username already exists".to_string()));
            }
        }

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

        // Store user
        {
            let mut users = self.users.lock()
                .map_err(|_| AppError::InternalError("Failed to acquire users lock".to_string()))?;
            
            let mut email_index = self.email_index.lock()
                .map_err(|_| AppError::InternalError("Failed to acquire email index lock".to_string()))?;
            
            let mut username_index = self.username_index.lock()
                .map_err(|_| AppError::InternalError("Failed to acquire username index lock".to_string()))?;

            users.insert(user_id, user.clone());
            email_index.insert(request.email, user_id);
            username_index.insert(request.username, user_id);
        }

        Ok(user.into())
    }

    pub fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<UserResponse>> {
        let users = self.users.lock()
            .map_err(|_| AppError::InternalError("Failed to acquire users lock".to_string()))?;
        
        Ok(users.get(&user_id).map(|user| user.clone().into()))
    }

    pub fn get_user_by_email(&self, email: &str) -> Result<Option<UserResponse>> {
        let email_index = self.email_index.lock()
            .map_err(|_| AppError::InternalError("Failed to acquire email index lock".to_string()))?;
        
        if let Some(&user_id) = email_index.get(email) {
            self.get_user_by_id(user_id)
        } else {
            Ok(None)
        }
    }
}
