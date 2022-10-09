mod quran_version;
mod page_result;
mod transporter;
mod config;

use std::error::Error;
use quran_version::QuranVersion;
use page_result::PageResult;
pub use config::Config;

rust_i18n::i18n!("locales");


pub fn run_once(config: &Config) -> Result<(), Box<dyn Error>> {
    let page = &config.quran_version.get_random_page().unwrap();
    for transport in config.transports.iter() {
        transport.send(&page.file_path, &config.language).unwrap();
    }
    Ok(())
}
