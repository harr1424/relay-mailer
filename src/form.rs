use ammonia::clean;
use serde::Deserialize;
use validator::Validate;

/// A struct describing expected form data and its validation rules
/// - name: A string describing the contact's name with a minimum length of
/// 1 and a maximum length of 30
/// - country: A string describing the contact's country with a minimum length of
/// 1 and a maximum length of 30
/// - email: A string describing the contact's email address that must be a valid email
/// - message: A string representation of the contact's message with a minimum length of 1
/// - language: An optional string describing the contact's preferred language
#[derive(Debug, Validate, Deserialize, Clone)]
pub struct FormData {
    #[validate(length(max = 30))]
    pub name: String,

    #[validate(length(max = 30))]
    pub country: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub message: String,

    pub language: Option<String>,
}

/// Sanitize form data by cleaning all fields using the defaults provided by the ammonia crate
impl FormData {
    pub fn sanitize(&mut self) {
        self.name = clean(&self.name).to_string();
        self.country = clean(&self.country).to_string();
        self.email = clean(&self.email).to_string();
        self.message = clean(&self.message).to_string();
        if let Some(language) = &self.language {
            self.language = Some(clean(language).to_string());
        }
    }
}
