use actix_web::{post, get, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::{SignupUser, LoginUser}; // Ensure these structs are defined in your models module
use crate::db; // Database module for user operations
use crate::email; // Email module for sending verification emails
use validator::Validate; // Validation for user input

// Handler for user signup
#[post("/signup")]
async fn signup(
    pool: web::Data<PgPool>,           // Dependency injection for PostgreSQL connection pool
    user: web::Json<SignupUser>,       // JSON request payload for signup
) -> impl Responder {
    // Validate the incoming SignupUser data
    if let Err(errors) = user.validate() {
        return HttpResponse::BadRequest().json(errors); // Return validation errors
    }

    // Create user in the database
    match db::create_user(&pool, &user.email, &user.password).await {
        Ok(user) => {
            // Construct the verification URL
            let verification_url = format!("http://localhost:8080/verify/{}", user.verification_token);
            // Send verification email
            if let Err(e) = email::send_verification_email(&user.email, &verification_url).await {
                println!("Failed to send verification email: {}", e);
                return HttpResponse::InternalServerError().json("Failed to send verification email");
            }
            // Success response
            HttpResponse::Ok().json("User created successfully. Please check your email for verification.")
        }
        Err(e) => {
            // Error handling for user creation failure
            HttpResponse::InternalServerError().json(format!("Failed to create user: {}", e))
        }
    }
}

// Handler for user login
#[post("/login")]
async fn login(
    pool: web::Data<PgPool>,           // Dependency injection for PostgreSQL connection pool
    user: web::Json<LoginUser>,        // JSON request payload for login
) -> impl Responder {
    // Validate the incoming LoginUser data
    if let Err(errors) = user.validate() {
        return HttpResponse::BadRequest().json(errors); // Return validation errors
    }

    // Authenticate the user
    match db::authenticate_user(&pool, &user.email, &user.password).await {
        Ok(token) => HttpResponse::Ok().json(token), // Return the JWT token on successful login
        Err(e) => HttpResponse::Unauthorized().json(format!("Authentication failed: {}", e)), // Error response
    }
}

// Handler for email verification
#[get("/verify/{token}")]
async fn verify_email(
    pool: web::Data<PgPool>,           // Dependency injection for PostgreSQL connection pool
    token: web::Path<String>,          // Path parameter for the verification token
) -> impl Responder {
    // Verify the email using the provided token
    match db::verify_email(&pool, &token).await {
        Ok(_) => HttpResponse::Ok().json("Email verified successfully"), // Success response
        Err(e) => HttpResponse::BadRequest().json(format!("Failed to verify email: {}", e)), // Error response
    }
}
