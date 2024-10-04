use actix_web::{test, web, App};
use auth_api::auth::{signup, login};
use auth_api::models::{SignupUser, LoginUser};
use sqlx::postgres::PgPoolOptions;

#[actix_rt::test]
async fn test_signup() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/testdb")
        .await
        .expect("Failed to create pool");

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(signup)
    ).await;

    let req = test::TestRequest::post()
        .uri("/signup")
        .set_json(&SignupUser {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        })
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&mut app, req).await;
    assert_eq!(resp["message"], "User created successfully. Please check your email for verification.");
}

#[actix_rt::test]
async fn test_login() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/testdb")
        .await
        .expect("Failed to create pool");

    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(login)
    ).await;

    let req = test::TestRequest::post()
        .uri("/login")
        .set_json(&LoginUser {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        })
        .to_request();

    let resp: serde_json::Value = test::call_and_read_body_json(&mut app, req).await;
    assert!(resp.get("token").is_some());
}
