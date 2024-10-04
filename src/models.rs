//mod User;

//pub mod User;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
    pub is_verified: bool,
    pub verification_token: String,
}

#[derive(Deserialize, Validate)]
pub struct SignupUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
