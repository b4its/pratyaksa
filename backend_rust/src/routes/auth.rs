use crate::{
    config::AppConfig,
    db::postgres::PostgresDb,
    errors::AppError,
    middleware::auth::extract_claims,
    models::auth::{AuthResponse, Claims, LoginRequest, RegisterRequest, UserPublic},
};
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;
use validator::Validate;

pub async fn register(
    db: web::Data<PostgresDb>,
    config: web::Data<AppConfig>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check if email already exists
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE email = $1"
    )
    .bind(&body.email)
    .fetch_one(&db.pool)
    .await?;

    if existing > 0 {
        return Err(AppError::Conflict("Email sudah terdaftar".to_string()));
    }

    // Hash password
    let password_hash = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST)?;

    // Insert user
    let user = sqlx::query_as::<_, crate::models::auth::User>(
        r#"
        INSERT INTO users (id, name, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'user', NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(&body.name)
    .bind(&body.email)
    .bind(&password_hash)
    .fetch_one(&db.pool)
    .await?;

    let token = create_token(&user.id.to_string(), &user.email, &user.role, &config)?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "data": AuthResponse {
            token,
            user: UserPublic::from(user),
        }
    })))
}

pub async fn login(
    db: web::Data<PostgresDb>,
    config: web::Data<AppConfig>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, AppError> {
    body.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Find user by email
    let user = sqlx::query_as::<_, crate::models::auth::User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&body.email)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Email atau password salah".to_string()))?;

    // Verify password
    let valid = bcrypt::verify(&body.password, &user.password_hash)
        .map_err(|_| AppError::InternalError("Password verification failed".to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized("Email atau password salah".to_string()));
    }

    let token = create_token(&user.id.to_string(), &user.email, &user.role, &config)?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": AuthResponse {
            token,
            user: UserPublic::from(user),
        }
    })))
}

pub async fn me(
    db: web::Data<PostgresDb>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let claims = extract_claims(&req)?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".to_string()))?;

    let user = sqlx::query_as::<_, crate::models::auth::User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(&db.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User tidak ditemukan".to_string()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": UserPublic::from(user),
    })))
}

fn create_token(
    user_id: &str,
    email: &str,
    role: &str,
    config: &AppConfig,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp() as usize;
    let exp = now + (config.jwt_expiry_hours as usize * 3600);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::InternalError(format!("Token generation failed: {}", e)))
}
