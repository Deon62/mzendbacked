use crate::errors::{AppError, Result};
use crate::models::user::User;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use std::path::Path;

pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn new(database_path: &str) -> Result<Self> {
        // Ensure the directory exists
        if let Some(parent) = Path::new(database_path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::DatabaseError(format!("Failed to create database directory: {}", e)))?;
        }

        // Create the database file if it doesn't exist
        if !Path::new(database_path).exists() {
            std::fs::File::create(database_path)
                .map_err(|e| AppError::DatabaseError(format!("Failed to create database file: {}", e)))?;
            println!("📁 Created new database file: {}", database_path);
        }

        let database_url = format!("sqlite:{}", database_path);
        
        let pool = SqlitePool::connect(&database_url)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to connect to database: {}", e)))?;

        let db = Self { pool };
        
        // Create tables if they don't exist
        db.create_tables().await?;
        
        println!("✅ Connected to SQLite database: {}", database_path);
        Ok(db)
    }

    async fn create_tables(&self) -> Result<()> {
        let query = r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT UNIQUE NOT NULL,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                is_verified BOOLEAN DEFAULT FALSE,
                stellar_public_key TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
            CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
        "#;

        sqlx::query(query)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to create tables: {}", e)))?;

        println!("📋 Database tables created/verified");
        Ok(())
    }

    pub async fn create_user(&self, user: &User) -> Result<()> {
        let query = r#"
            INSERT INTO users (id, email, username, password_hash, is_verified, stellar_public_key, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        "#;

        sqlx::query(query)
            .bind(user.id.to_string())
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password_hash)
            .bind(user.is_verified)
            .bind(&user.stellar_public_key)
            .bind(user.created_at.to_rfc3339())
            .bind(user.updated_at.to_rfc3339())
            .execute(&self.pool)
            .await
            .map_err(|e| {
                if e.to_string().contains("UNIQUE constraint failed") {
                    if e.to_string().contains("email") {
                        AppError::ValidationError("Email already exists".to_string())
                    } else if e.to_string().contains("username") {
                        AppError::ValidationError("Username already exists".to_string())
                    } else {
                        AppError::ValidationError("User already exists".to_string())
                    }
                } else {
                    AppError::DatabaseError(format!("Failed to create user: {}", e))
                }
            })?;

        println!("💾 User '{}' saved to database", user.username);
        Ok(())
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let query = "SELECT * FROM users WHERE email = ?1";

        let row = sqlx::query(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to fetch user by email: {}", e)))?;

        if let Some(row) = row {
            Ok(Some(User {
                id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
                email: row.get("email"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                is_verified: row.get("is_verified"),
                stellar_public_key: row.get("stellar_public_key"),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>> {
        let query = "SELECT * FROM users WHERE username = ?1";

        let row = sqlx::query(query)
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to fetch user by username: {}", e)))?;

        if let Some(row) = row {
            Ok(Some(User {
                id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
                email: row.get("email"),
                username: row.get("username"),
                password_hash: row.get("password_hash"),
                is_verified: row.get("is_verified"),
                stellar_public_key: row.get("stellar_public_key"),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_count(&self) -> Result<i64> {
        let query = "SELECT COUNT(*) as count FROM users";
        
        let row = sqlx::query(query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to get user count: {}", e)))?;

        Ok(row.get("count"))
    }
}
