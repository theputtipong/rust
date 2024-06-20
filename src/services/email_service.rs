use lettre::message::Mailbox;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send_email(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from("your_email@gmail.com".parse().unwrap())
        .to(Mailbox::new(None, to.parse()?))
        .subject(subject)
        .body(body.to_string())?;

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(("your_email@gmail.com", "your_email_password").into())
        .build();

    mailer.send(&email)?;

    Ok(())
}
