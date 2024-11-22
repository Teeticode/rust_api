use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]

pub enum UserRole {
    Admin,
    User,
    Landlord,
}

impl UserRole {
    pub fn to_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
            UserRole::Landlord => "landlord",
        }
    }
}
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, sqlx::Type, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
    pub role: UserRole,
    pub verified: bool,
    pub verification_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updateAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
    pub phone: String,
}

#[derive(Deserialize)]
pub struct VerifyOtp {
    pub phone: String,
    pub code: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OTPVerifyResponse {
    pub status: String,
}
