use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// User record stored in PostgreSQL
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Public user info returned in API responses (no password)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserPublic {
    fn from(u: User) -> Self {
        UserPublic {
            id: u.id,
            name: u.name,
            email: u.email,
            role: u.role,
            created_at: u.created_at,
        }
    }
}

/// Request body for POST /auth/register
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 100, message = "Nama harus antara 2-100 karakter"))]
    pub name: String,
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: String,
}

/// Request body for POST /auth/login
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

/// Response body for successful auth
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
}

/// JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   // user UUID
    pub email: String,
    pub role: String,
    pub exp: usize,    // expiry timestamp
    pub iat: usize,    // issued at
}
