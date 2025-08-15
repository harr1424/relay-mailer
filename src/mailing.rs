use actix_web::{post, web, HttpResponse};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, info};
use std::sync::Arc;
use validator::Validate;

use crate::form::FormData;
use crate::config::Config;

/// Create a subject line based on the language of the form data
/// - English => "Answer to your question in English from {country}"
/// - Español => "Respuesta a su pregunta en español desde {country}"
/// - Français => "Répondez à votre question en français de {country}"
/// - Português => "Responda a sua pergunta em português da {country}"
/// - Italiano => "Rispondi alla tua domanda in italiano dal {country}"
/// - Deutsch => "Antworten zu Ihrer Frage auf Deutsch von {country}"
pub fn create_subject_by_lang(form: FormData) -> String {
    match form
        .language
        .clone()
        .unwrap_or(String::from("UNKNOWN LANGUAGE"))
        .as_str()
    {
        "English" => String::from(format!(
            "Answer to your question in English from {}",
            form.country
        )),
        "Español" => String::from(format!(
            "Respuesta a su pregunta en español desde {}",
            form.country
        )),
        "Français" => String::from(format!(
            "Répondez à votre question en français de {}",
            form.country
        )),
        "Português" => String::from(format!(
            "Responda a sua pergunta em português da {}",
            form.country
        )),
        "Italiano" => String::from(format!(
            "Rispondi alla tua domanda in italiano dal {}",
            form.country
        )),
        "Deutsch" => String::from(format!(
            "Antworten zu Ihrer Frage auf Deutsch von {}",
            form.country
        )),
        "UNKNOWN LANGUAGE" => String::from(format!(
            "Unable to detwermine language, the listener is from {}",
            form.country
        )),
        _ => String::from("Unable to determine language and location"),
    }
}

/// Validate and sanitize form field data and send it to the forward_address provided in Config.toml
/// 
/// # Panics 
/// Validation is enforced gracefully, but if form input should pass validation and still
/// fail to parse into a format string this function will panic.
/// 
/// If any strings present in Config.toml fail to parse into a valid values this function will panic.
/// 
/// For example user and forward_address must be valid email addresses, and listen_address must be a valid 
/// IPv4 address with port number. 
#[post("/contact")]
pub async fn relay_message(config: web::Data<Arc<Config>>, mut form: web::Form<FormData>) -> HttpResponse {
    if let Err(errors) = form.validate() {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .map(|(field, _)| format!("Invalid input in the {} field", field))
            .collect();

        return HttpResponse::BadRequest().json(error_messages);
    }

    form.sanitize();

    info!(
        "Received contact from {} in {} from email {}",
        form.name, form.country, form.email
    );

    let subject = create_subject_by_lang(form.clone());

    let email = Message::builder()
        .from(form.email.parse::<Mailbox>().expect("Failed to parse sender email from form data"))
        .reply_to(form.email.parse::<Mailbox>().expect("Failed to parse reply-to email from form data"))
        .to(config.forward_address.parse().expect("Failed to parse forward_address provided in Config.toml"))
        .subject(subject)
        .body(format!(
            "Name: {}\n\nLocation: {}\n\nEmail: {}\n\nMessage: {}",
            form.name, form.country, form.email, form.message
        ))
        .expect("Failed to build email message from form data");

    let mailer = SmtpTransport::relay(&config.server)
        .expect("Failed to parse relay server provided in Config.toml")
        .credentials(Credentials::from((
            config.user.clone(),
            config.pwd.clone(),
        )))
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            info!("Message from {} was forwarded", form.email);
            HttpResponse::Ok().body("Message sent successfully")
        }
        Err(e) => {
            error!("FAILED to send message to {}", form.email);
            HttpResponse::InternalServerError()
                .body(format!("Failed to send message: {}", e.to_string()))
        }
    }
}
