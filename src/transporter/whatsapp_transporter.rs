use super::transport::{format_secret, Transport};
use derivative::Derivative;
use reqwest::blocking::{multipart::Form, Client};
use rust_i18n::t;
use std::{collections::HashMap, env, error::Error};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct WhatsappTransporter {
    version: String,
    sender_phone_id: String,
    recipients: Vec<String>,
    #[derivative(Debug(format_with = "format_secret"))]
    access_token: String,
}

impl WhatsappTransporter {
    fn get_url(&self, path: &str) -> String {
        format!(
            "https://graph.facebook.com/{}/{}{}",
            self.version, self.sender_phone_id, path
        )
    }

    fn upload_image(&self, attachment: &str) -> Result<String, Box<dyn Error>> {
        let form_data = Form::new()
            .text("messaging_product", "whatsapp")
            .file("file", attachment)?;

        let mut resp = Client::new()
            .post(self.get_url("/media"))
            .bearer_auth(&self.access_token)
            .multipart(form_data)
            .send()?
            .json::<HashMap<String, String>>()?;

        Ok(resp.remove("id").unwrap())
    }
}

impl Transport for WhatsappTransporter {
    fn send(&self, attachment: &str, language: &str) -> Result<(), Box<dyn Error>> {
        let image_id = self.upload_image(attachment)?;
        for recipient in self.recipients.iter() {
            Client::new()
                .post(self.get_url("/messages"))
                .bearer_auth(&self.access_token)
                .json(&serde_json::json!({
                            "messaging_product": "whatsapp",
                            "recipient_type": "individual",
                            "type": "image",
                            "to": &recipient,
                            "image": serde_json::json!({
                                "id": &image_id,
                                "caption": t!("daily_sms.caption", locale = language)
                            })
                }))
                .send()?
                .error_for_status()?;
        }
        Ok(())
    }

    fn from_env() -> Result<Box<Self>, Box<dyn Error>> {
        let access_token = env::var("WHATSAPP_ACCESS_TOKEN")?;
        let sender_phone_id = env::var("WHATSAPP_SENDER_PHONE_ID")?;
        let recipients = env::var("RECIPIENT_PHONES")?;
        let recipients = recipients
            .split(",")
            .map(|st| st.to_owned())
            .collect::<Vec<_>>();
        Ok(Box::new(WhatsappTransporter {
            access_token,
            version: String::from("v15.0"),
            sender_phone_id,
            recipients,
        }))
    }
}
