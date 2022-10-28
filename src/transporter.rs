mod email_transporter;
mod transport;
mod whatsapp_transporter;

use derivative::Derivative;
use email_transporter::EmailTransporter;
use std::{env, error::Error};
use transport::Transport;
use whatsapp_transporter::WhatsappTransporter;

#[derive(Derivative)]
#[derivative(Debug = "transparent")]
pub struct Transporter(Box<dyn Transport>);

impl Transporter {
    fn get_enabled_transports() -> Vec<String> {
        env::var("TRANSPORTS")
            .unwrap_or("email, whatsapp".to_string())
            .split(",")
            .map(|s| s.trim().to_owned())
            .collect()
    }
    pub fn from_env() -> Vec<Transporter> {
        let enabled_transports = Self::get_enabled_transports();
        println!("enabled_transports: {enabled_transports:#?}");

        let mut transports: Vec<Transporter> = Vec::with_capacity(2);
        if enabled_transports.contains(&"email".to_string()) {
            if let Ok(email_transporter) = EmailTransporter::from_env() {
                transports.push(Transporter(email_transporter));
            } else {
                eprintln!("failed to parse email_config");
            }
        }

        if enabled_transports.contains(&"whatsapp".to_string()) {
            if let Ok(whatsapp_transporter) = WhatsappTransporter::from_env() {
                transports.push(Transporter(whatsapp_transporter));
            } else {
                eprintln!("failed to parse whatsapp_config");
            }
        }

        transports
    }

    pub fn send(&self, attachment: &str, language: &str) -> Result<(), Box<dyn Error>> {
        self.0.send(attachment, language)
    }
}
