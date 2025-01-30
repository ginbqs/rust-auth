use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
// For sqlx
pub struct UserModel{
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    // pub token: Option<String>,
    // pub token_refresh: Option<String>,
    // pub password: Option<String>,
    pub is_verified: i8,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UserModelResponse {
    pub id: String,
    pub name: String,
    pub username: String,
    pub email: String,
    // pub token: Option<String>,
    // pub token_refresh: Option<String>,
    // pub password: Option<String>,
    pub is_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>
}