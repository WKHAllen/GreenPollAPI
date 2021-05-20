use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;
use std::io::{Result, Error, ErrorKind};
use std::collections::HashMap;

/// Sends an email using the app email account
/// 
/// # Arguments
/// 
/// * `email_to` - The address to send the email to
/// * `email_subject` - The subject line of the email
/// * `email_html` - The HTML content to send in the email body
/// * `email_text` - The text content to send in the email body
pub fn send_email(
    email_to: String,
    email_subject: String,
    email_html: String,
    email_text: String
) -> Result<lettre::smtp::response::Response> {
    let email_address = std::env::var("EMAIL_ADDRESS")
        .expect("EMAIL_ADDRESS must exist");    
    let email_password = std::env::var("EMAIL_APP_PASSWORD")
        .expect("EMAIL_APP_PASSWORD must exist");

    let email = match EmailBuilder::new()
        .from((email_address.clone(), "GreenPoll"))
        .to(email_to)
        .subject(email_subject)
        .alternative(email_html, email_text)
        .build() {
            Ok(val) => Ok(val),
            Err(e) => Err(Error::new(ErrorKind::Other, format!("Failed to build email: {}", e)))
        }?;

    let creds = Credentials::new(email_address.clone(), email_password.clone());

    let mut mailer = match SmtpClient::new_simple("smtp.gmail.com") {
        Ok(val) => Ok(val),
        Err(e) => Err(Error::new(ErrorKind::Other, format!("Failed to create email client: {}", e)))
    }?
        .credentials(creds)
        .transport();

    let result = match mailer.send(email.into()) {
        Ok(val) => Ok(val),
        Err(e) => Err(Error::new(ErrorKind::Other, format!("Failed to send email: {}", e)))
    }?;

    Ok(result)
}

/// Sends an email in the `/emails` directory, replacing placeholder values in the HTML and text representations of the email
/// 
/// # Arguments
/// 
/// * `email_to` - The address to send the email to
/// * `email_subject` - The subject line of the email
/// * `email_name` - The name of the email files in the emails directory
/// * `options` - Key-value pairs of the placeholders and their values
/// 
/// # Placeholder example
/// 
/// In the HTML and text email files, the following could be used to insert placeholder values:
/// 
/// `Hello, {firstname} {lastname}. You have received the message: {message}.
/// 
/// The `options` parameter could then contain values for `firstname`, `lastname`, and `message`.
pub fn send_formatted_email(
    email_to: String,
    email_subject: String,
    email_name: String,
    options: HashMap<&str, &str>
) -> Result<lettre::smtp::response::Response> {
    let mut email_html = std::fs::read_to_string(format!("emails/{}.html", email_name))?;
    let mut email_text = std::fs::read_to_string(format!("emails/{}.txt", email_name))?;

    for (key, value) in options.iter() {
        let key_search = format!("{{{}}}", key);
        email_html = str::replace(&email_html[..], &key_search[..], value);
        email_text = str::replace(&email_text[..], &key_search[..], value);
    }

    let result = send_email(email_to, email_subject, email_html, email_text)?;

    Ok(result)
}
