use serde::{Deserialize, Serialize}; // Import necessary traits for serialization
use validator::Validate; // Import for validating user input
use uuid::Uuid; // For using Uuid for the user ID

// Define the User struct that represents a user in the database
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,                  // Unique identifier for the user
    pub email: String,             // User's email address
    pub password_hash: String,     // Hashed password
    pub verification_token: String, // Token for email verification
    pub is_verified: bool,         // Flag indicating if the email is verified
}

// Struct for user signup payload
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SignupUser {
    #[validate(email)] // Validate that the email is properly formatted
    pub email: String, // User's email address
    #[validate(length(min = 6))] // Validate that the password is at least 6 characters
    pub password: String, // User's password
}

// Struct for user login payload
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUser {
    #[validate(email)] // Validate that the email is properly formatted
    pub email: String, // User's email address
    #[validate(length(min = 6))] // Validate that the password is at least 6 characters
    pub password: String, // User's password
}
