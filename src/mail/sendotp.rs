use lettre::{
    message::{header, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

use std::{env, fs};

pub async fn send_otp(to_email: &str, subject: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;
    let smtp_server = env::var("SMTP_SERVER")?;
    let smtp_secret = env::var("SMTP_SECRET_KEY")?;

    // let mut html_template = fs::read_to_string(template_path)?;

    // for (key, value) in placeholders {
    //     html_template = html_template.replace(key, value)
    // }

    let email = Message::builder()
        .from("karanikateeti@gmail.com".parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Welcome to Keja")
        .body(String::from("1547436 test otp for you"))
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_secret);

    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    let result = mailer.send(&email);

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(err) => println!("Error sending email: {:?}", err),
    }

    Ok(())
}
