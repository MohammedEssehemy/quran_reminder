use std::{env, error::Error};

use super::{Transport, TransportFromEnv};
use crate::page_result::PageResult;
use derivative::Derivative;
use rust_i18n::t;
use sendgrid::v3::{Attachment, Content, Email, Message, Personalization, Sender};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct EmailTransporter {
    from: String,
    #[derivative(Debug = "ignore")]
    api_key: String,
    recipients: Vec<String>,
}

impl Transport for EmailTransporter {
    fn send(&self, page: &PageResult, language: &str) -> Result<(), Box<dyn Error>> {
        let sg = Sender::new(self.api_key.to_string());
        let sender_email = Email::new(self.from.to_string());
        let mut message = Message::new(sender_email)
            .set_subject(&t!("daily_email.subject", locale = language))
            .add_content(
                Content::new()
                    .set_content_type("text/html")
                    .set_value(t!("daily_email.content", locale = language)),
            )
            .add_attachment(
                Attachment::new()
                    .set_content(&page.content)
                    .set_filename(format!("{}.jpg", page.no))
                    .set_mime_type(page.mime_type.to_string()),
            );
        for recipient in self.recipients.iter() {
            message = message.add_personalization(Personalization::new(Email::new(recipient)));
        }
        // println!("{:?}", serde_json::to_string(&message));
        sg.send(&message)?;
        Ok(())
    }
}

impl TransportFromEnv for EmailTransporter {
    fn from_env() -> Result<Box<dyn Transport>, Box<dyn Error>> {
        let api_key = env::var("SENDGRID_API_KEY")?;
        let from = env::var("SENDGRID_FROM")?;
        let recipients = env::var("RECIPIENT_EMAILS")?;
        let recipients = recipients
            .split(",")
            .map(|st| String::from(st))
            .collect::<Vec<_>>();
        Ok(Box::new(EmailTransporter {
            api_key,
            from,
            recipients,
        }))
    }
}