use std::env;


#[derive(Debug)]
pub enum QuranVersion {
    Madina,
    Tajweed
}

impl QuranVersion {
    fn parse(text: String) -> Self {
        match text.as_str() {
            "madina" => Self::Madina,
            "tajweed" => Self::Tajweed,
            _ => Self::Madina
        }
    }
    
    pub fn get_path(&self) -> String {
        match self {
            Self::Madina => "madina".to_owned(),
            Self::Tajweed => "tajweed".to_owned(),
        }
    }
}



#[derive(Debug)]
pub struct Config {
    pub email_from: String,
    pub email_api_key: String,
    pub recipients: Vec<String>,
    pub quran_version: QuranVersion,
    pub email_language: String,
}

impl Config {
    pub fn init() -> Config {
        let email_api_key = env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY env is required");
        let email_from = env::var("SENDGRID_FROM").expect("SENDGRID_FROM env is required");
        let recipients = env::var("RECIPIENT_EMAILS").expect("RECIPIENT_EMAILS env is required");
        let recipients = recipients.split(",").map(|st| String::from(st)).collect::<Vec<_>>();
        let email_language = env::var("EMAIL_LANGUAGE").unwrap_or("ar".to_string());
        let quran_version = QuranVersion::parse(env::var("QURAN_VERSION").unwrap_or_default());
        Config {
            email_from,
            email_api_key,
            recipients,
            quran_version,
            email_language,
        }
    }
}
