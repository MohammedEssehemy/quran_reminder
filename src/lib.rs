mod quran_version;
mod page_result;
mod transporter;
mod config;

use rand::Rng;
use std::{error::Error, io::Error as IoError};
use quran_version::QuranVersion;
use page_result::PageResult;
pub use config::Config;

rust_i18n::i18n!("locales");

const QURAN_NO_PAGES: u32 = 604;

fn get_quran_page(version: &QuranVersion) -> Result<PageResult, IoError> {
    let page_no = get_random_page_no();
    let file_path = format!("./assets/{}/{page_no}.png", version.get_path());
    Ok(PageResult {
        no: page_no,
        file_path,
    })
}

fn get_random_page_no() -> u32 {
    let mut rng = rand::thread_rng();
    let page_no = rng.gen_range(1..QURAN_NO_PAGES);
    return page_no;
}

pub fn run_once(config: &Config) -> Result<(), Box<dyn Error>> {
    let page = get_quran_page(&config.quran_version).expect("failed to get random page");
    for transport in config.transports.iter() {
        transport.send(&page.file_path, &config.language).unwrap();
    }
    Ok(())
}
