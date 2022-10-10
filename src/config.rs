use std::env;

use crate::transporter::Transporter;
use crate::QuranVersion;

#[derive(Debug)]
pub struct Config {
    pub quran_version: QuranVersion,
    pub language: String,
    pub transports: Vec<Transporter>,
}

impl Config {
    pub fn init() -> Config {
        let language = env::var("LANGUAGE").unwrap_or("ar".to_string());
        let quran_version = QuranVersion::from_env();
        let transports = Transporter::from_env();

        if transports.len() == 0 {
            panic!("at least one transport is required")
        }

        Config {
            transports,
            quran_version,
            language,
        }
    }
}
