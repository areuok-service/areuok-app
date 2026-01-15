//! External services for the areuok application.
//!
//! This module contains integrations with external APIs and services,
//! including email notifications and daily quote fetching.

use lettre::message::{header::ContentType, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::models::{EmailConfig, HitokotoResponse, Quote};

/// Fetch a daily inspirational quote from hitokoto.cn API
pub async fn fetch_hitokoto() -> Result<Quote, String> {
    log::info!("Fetching daily quote from hitokoto.cn API");
    let client = reqwest::Client::new();
    let url = "https://v1.hitokoto.cn/";

    let response = client.get(url).send().await.map_err(|e| {
        log::error!("Failed to send request to hitokoto.cn: {}", e);
        format!("Failed to send request: {}", e)
    })?;

    let hitokoto_response: HitokotoResponse = response.json().await.map_err(|e| {
        log::error!("Failed to parse hitokoto response: {}", e);
        format!("Failed to parse response: {}", e)
    })?;

    let author = hitokoto_response
        .from_who
        .unwrap_or_else(|| hitokoto_response.from.clone());

    log::info!(
        "Successfully fetched daily quote: \"{}\" - {}",
        hitokoto_response.hitokoto,
        author
    );

    Ok(Quote {
        text: hitokoto_response.hitokoto,
        author,
    })
}

/// Send a sign-in notification email
pub fn send_signin_email(
    name: &str,
    streak: i32,
    quote: &Quote,
    config: &EmailConfig,
) -> Result<(), String> {
    if !config.enabled || config.to_email.is_empty() {
        log::debug!("Email notification disabled or recipient email not configured");
        return Ok(());
    }

    log::info!("Preparing sign-in email notification for {} (streak: {} days)", name, streak);

    let from = parse_email_address(&config.from_email, "from")?;
    let to = parse_email_address(&config.to_email, "to")?;

    let subject = format!("🔥 {} 签到成功！连续签到 {} 天", name, streak);
    let body = build_email_body(name, streak, quote);

    let email = build_email_message(from, to, &subject, body)?;
    send_via_smtp(email, config)
}

/// Parse and validate an email address
fn parse_email_address(email: &str, field_name: &str) -> Result<Mailbox, String> {
    email.parse::<Mailbox>().map_err(|e| {
        log::error!("Invalid {} email address '{}': {}", field_name, email, e);
        format!("Invalid {} email: {}", field_name, e)
    })
}

/// Build the email body content
fn build_email_body(name: &str, streak: i32, quote: &Quote) -> String {
    format!(
        "Hi {},\n\n\
        恭喜你今天成功签到！🎉\n\n\
        当前连续签到天数：{} 天 🔥\n\n\
        每日一言：\n\
        \"{}\"\n\
        - {}\n\n\
        继续保持，加油！💪\n\n\
        --\n\
        Are You OK?",
        name, streak, quote.text, quote.author
    )
}

/// Build the email message
fn build_email_message(
    from: Mailbox,
    to: Mailbox,
    subject: &str,
    body: String,
) -> Result<Message, String> {
    Message::builder()
        .from(from)
        .to(to)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .map_err(|e| {
            log::error!("Failed to build email message: {}", e);
            format!("Failed to build email: {}", e)
        })
}

/// Send email via SMTP
fn send_via_smtp(email: Message, config: &EmailConfig) -> Result<(), String> {
    log::debug!("Connecting to SMTP server: {}:{}", config.smtp_server, config.smtp_port);

    let credentials = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    let mailer = SmtpTransport::starttls_relay(&config.smtp_server)
        .map_err(|e| {
            log::error!("Failed to create SMTP relay for {}: {}", config.smtp_server, e);
            format!("Failed to create SMTP relay: {}", e)
        })?
        .port(config.smtp_port)
        .credentials(credentials)
        .build();

    mailer.send(&email).map_err(|e| {
        log::error!("Failed to send email via SMTP: {}", e);
        format!("Failed to send email: {}", e)
    })?;

    log::info!("Successfully sent sign-in email notification to {}", config.to_email);
    Ok(())
}
