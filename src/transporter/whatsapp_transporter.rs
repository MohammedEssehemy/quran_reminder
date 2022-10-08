use derivative::Derivative;
use std::{env, error::Error};

use super::{Transport, TransportFromEnv};
use crate::page_result::PageResult;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct WhatsappTransporter {
    #[derivative(Debug = "ignore")]
    access_token: String,
    version: i8,
    sender_phone_id: String,
    recipients: Vec<String>,
}

impl TransportFromEnv for WhatsappTransporter {
    fn from_env() -> Result<Box<dyn Transport>, Box<dyn Error>> {
        let access_token = env::var("WHATSAPP_ACCESS_TOKEN")?;
        let version = 15;
        let sender_phone_id = env::var("WHATSAPP_SENDER_PHONE_ID")?;
        let recipients = env::var("RECIPIENT_PHONES")?;
        let recipients = recipients
            .split(",")
            .map(|st| String::from(st))
            .collect::<Vec<_>>();
        Ok(Box::new(WhatsappTransporter {
            access_token,
            version,
            sender_phone_id,
            recipients,
        }))
    }
}

impl Transport for WhatsappTransporter {
    fn send(&self, page: &PageResult, language: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
