# User Authentication System

This is a Rust-based User Authentication System that allows users to sign up, log in, and verify their email addresses. The application utilizes PostgreSQL for data storage, Bcrypt for password hashing, and JSON Web Tokens (JWT) for user authentication.

## Table of Contents

- [Features](#features)
- [Technologies Used](#technologies-used)
- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
  - [Sign Up](#sign-up)
  - [Log In](#log-in)
  - [Email Verification](#email-verification)
- [Database Setup](#database-setup)
- [Environment Variables](#environment-variables)
- [Running the Application](#running-the-application)
- [License](#license)

## Features

- User sign-up with email verification
- Secure password hashing using Bcrypt
- JWT token generation for authenticated sessions
- RESTful API for easy integration

## Technologies Used

- **Rust**: Programming language used for backend development
- **Actix-web**: Web framework for building the API
- **SQLx**: Asynchronous SQL toolkit for Rust
- **PostgreSQL**: Relational database for data storage
- **Bcrypt**: Library for password hashing
- **JSON Web Tokens (JWT)**: Used for authentication

## Getting Started

### Prerequisites

- Rust installed (use [rustup](https://rustup.rs/) to install)
- PostgreSQL installed and running
- `cargo` for managing Rust packages

### Clone the Repository

```bash
git clone https://github.com/your-username/your-repo-name.git
cd your-repo-name
```

## API Endpoints

### Sign Up

- **Endpoint**: `POST /signup`
- **Request Body**: 
```json
{
    "email": "user@example.com",
    "password": "yourpassword"
}
```
- **Response**: 
  - **200 OK**: User created successfully. Verification email sent.
  - **400 Bad Request**: Validation errors.
  - **500 Internal Server Error**: Server error.

### Log In

- **Endpoint**: `POST /login`
- **Request Body**:
```json
{
    "email": "user@example.com",
    "password": "yourpassword"
}
```
- **Response**: 
  - **200 OK**: Returns a JWT token.
  - **401 Unauthorized**: Invalid credentials or email not verified.
  
### Email Verification

- **Endpoint**: `GET /verify/{token}`
- **Response**: 
  - **200 OK**: Email verified successfully.
  - **400 Bad Request**: Invalid verification token.

## Database Setup

To set up the PostgreSQL database:

1. Create a new database:

```sql
CREATE DATABASE your_database_name;
```

2. Run the necessary migrations or create the users table with the following SQL:

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    verification_token VARCHAR(255) NOT NULL,
    is_verified BOOLEAN DEFAULT FALSE
);
```

## Environment Variables

Ensure the following environment variables are set in your `.env` file:

```plaintext
DATABASE_URL=postgres://username:password@localhost/your_database_name
JWT_SECRET=your_jwt_secret_key
```

## Running the Application

To run the application, use the following command:

```bash
cargo run
```

The server will start at `http://localhost:8080`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

