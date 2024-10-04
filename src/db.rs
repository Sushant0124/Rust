use sqlx::PgPool;
use crate::models::User; // Ensure the User struct is defined in your models module
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;
use serde::{Serialize, Deserialize}; // Import serde traits for serialization
use chrono; // For handling dates and times

// Function to create a new user in the database
// Function to create a new user in the database
pub async fn create_user(pool: &PgPool, email: &str, password: &str) -> Result<User, sqlx::Error> {
    // Hash the user's password
    let hashed_password = hash(password, DEFAULT_COST).map_err(|e| {
        eprintln!("Error hashing password: {}", e);
        sqlx::Error::RowNotFound // Change this to an appropriate error type
    })?;
    
    // Generate a verification token for the user
    let verification_token = Uuid::new_v4().to_string();

    // Insert the new user into the database and return the user
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password_hash, verification_token, is_verified) VALUES ($1, $2, $3, $4) RETURNING *",
        email,
        hashed_password,
        verification_token,
        false // Pass `false` directly for the is_verified column
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}


// Function to authenticate a user and return a JWT token if successful
pub async fn authenticate_user(pool: &PgPool, email: &str, password: &str) -> Result<String, String> {
    // Fetch the user from the database based on the provided email
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    if let Some(user) = user {
        if !user.is_verified {
            return Err("Email not verified".to_string());
        }

        // Verify the provided password against the stored hash
        if verify(password, &user.password_hash).map_err(|e| format!("Password verification error: {}", e))? {
            let token = create_jwt(&user.id.to_string())?; // Create JWT here
            Ok(token)
        } else {
            Err("Invalid password".to_string())
        }
    } else {
        Err("User not found".to_string())
    }
}

// Function to verify a user's email based on the verification token
pub async fn verify_email(pool: &PgPool, token: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE users SET is_verified = true WHERE verification_token = $1",
        token
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Function to create a JWT for the user
fn create_jwt(user_id: &str) -> Result<String, String> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| format!("JWT creation error: {}", e))
}

// Models for JWT Claims
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,   // Subject (usually the user ID or email)
    exp: usize,    // Expiration time as a timestamp
}
