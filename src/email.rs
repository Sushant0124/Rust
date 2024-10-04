use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_verification_email(to: &str, verification_url: &str) -> Result<(), String> {
    let email = Message::builder()
        .from("noreply@example.com".parse().unwrap())
        .to(to.parse().unwrap())
        .subject("Verify your email")
        .body(format!("Please click the following link to verify your email: {}", verification_url))
        .unwrap();

    let creds = Credentials::new(
        "smtp_username".to_string(),
        "smtp_password".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Could not send email: {:?}", e)),
    }
}
