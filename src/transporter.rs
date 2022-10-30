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

macro_rules! init_transports_from_env {
    ($enabled_transports: expr, $(($key: literal, $struct:ty)),+) => {{
        let mut transports = vec![];
        $(
            if $enabled_transports.contains(&$key.to_string()) {
                match <$struct>::from_env() {
                    Ok(transport) => {
                        transports.push(Transporter(transport));
                    },
                    Err(err) => {
                        panic!("failed to init {:?} with error {}", stringify!($struct), err);
                    }
                }
            }
        )+
        transports
    }};
}

impl Transporter {
    fn get_enabled_transports() -> Vec<String> {
        env::var("TRANSPORTS")
            .unwrap_or("email, whatsapp".to_string())
            .split(",")
            .map(|s| s.trim().to_lowercase().to_owned())
            .collect()
    }
    pub fn from_env() -> Vec<Transporter> {
        let enabled_transports = Self::get_enabled_transports();
        println!("enabled_transports: {enabled_transports:#?}");

        let transports = init_transports_from_env!(
            enabled_transports,
            ("email", EmailTransporter),
            ("whatsapp", WhatsappTransporter)
        );

        transports
    }

    pub fn send(&self, attachment: &str, language: &str) -> Result<(), Box<dyn Error>> {
        self.0.send(attachment, language)
    }
}
