use rand::Rng;
use rust_i18n::t;
use sendgrid::v3::{Attachment, Content, Email, Message, Personalization, Sender};
use std::{error::Error, fs::read, io::Error as IoError};
pub mod config;
use config::{Config, QuranVersion};

rust_i18n::i18n!("locales");

const QURAN_NO_PAGES: u32 = 604;

struct PageResult {
    pub no: u32,
    pub content: Vec<u8>,
    pub mime_type: String,
}

fn get_quran_page(version: &QuranVersion) -> Result<PageResult, IoError> {
    let page_no = get_random_page_no();
    let file_path = format!("./assets/{}/{page_no}.png", version.get_path());
    let page = read(&file_path)?;
    Ok(PageResult {
        no: page_no,
        content: page,
        mime_type: String::from("image/jpg"),
    })
}

fn get_random_page_no() -> u32 {
    let mut rng = rand::thread_rng();
    let page_no = rng.gen_range(1..QURAN_NO_PAGES);
    return page_no;
}

pub fn run_once(config: &Config) -> Result<(), Box<dyn Error>> {
    let sg = Sender::new(config.email_api_key.clone());
    let page = get_quran_page(&config.quran_version).expect("failed to get random page");

    let sender_email = Email::new(config.email_from.clone());
    let mut message = Message::new(sender_email)
        .set_subject(&t!("daily_email.subject", locale = &config.email_language))
        .add_content(
            Content::new()
                .set_content_type("text/html")
                .set_value(t!("daily_email.content", locale = &config.email_language)),
        )
        .add_attachment(
            Attachment::new()
                .set_content(&page.content)
                .set_filename(format!("{}.jpg", page.no))
                .set_mime_type(page.mime_type),
        );
    for recipient in config.recipients.iter() {
        message = message.add_personalization(Personalization::new(Email::new(recipient)));
    }
    // println!("{:?}", serde_json::to_string(&message));
    sg.send(&message)?;
    Ok(())
}
