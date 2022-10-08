mod email_transporter;
mod transport;
mod whatsapp_transporter;

use crate::page_result::PageResult;
use derivative::Derivative;
use email_transporter::EmailTransporter;
use std::error::Error;
use transport::{Transport, TransportFromEnv};
use whatsapp_transporter::WhatsappTransporter;

#[derive(Derivative)]
#[derivative(Debug = "transparent")]
pub struct Transporter(Box<dyn Transport>);

impl Transporter {
    pub fn from_env() -> Vec<Transporter> {
        let mut transports: Vec<Transporter> = Vec::with_capacity(2);

        if let Ok(email_transporter) = EmailTransporter::from_env() {
            transports.push(Transporter(email_transporter));
        } else {
            eprintln!("failed to parse email_config");
        }

        if let Ok(whatsapp_transporter) = WhatsappTransporter::from_env() {
            transports.push(Transporter(whatsapp_transporter));
        } else {
            eprintln!("failed to parse whatsapp_config");
        }

        transports
    }

    pub fn send(&self, page: &PageResult, language: &str) -> Result<(), Box<dyn Error>> {
        self.0.send(page, language)
    }
}
